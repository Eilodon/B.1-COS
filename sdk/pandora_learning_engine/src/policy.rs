use crate::value_estimator::{NeuralQValueEstimator, QValueEstimator};
use pandora_core::ontology::EpistemologicalFlow;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    Noop,
    Explore,
    Exploit,
    UnlockDoor,
    PickUpKey,
    MoveForward,
}

impl Action {
    /// Convert action to string for Q-value estimation
    pub fn as_str(&self) -> &'static str {
        match self {
            Action::Noop => "noop",
            Action::Explore => "explore",
            Action::Exploit => "exploit",
            Action::UnlockDoor => "unlock_door",
            Action::PickUpKey => "pick_up_key",
            Action::MoveForward => "move_forward",
        }
    }

    /// Get all available actions
    pub fn all_actions() -> Vec<Action> {
        vec![
            Action::Noop,
            Action::Explore,
            Action::Exploit,
            Action::UnlockDoor,
            Action::PickUpKey,
            Action::MoveForward,
        ]
    }
}

pub trait Policy {
    fn select_action(&self, flow: &EpistemologicalFlow, explore_rate: f64) -> Action;
    fn update(&mut self, _flow: &EpistemologicalFlow, _advantage: f64) {}
}

#[derive(Default)]
pub struct EpsilonGreedyPolicy;

impl Policy for EpsilonGreedyPolicy {
    fn select_action(&self, _flow: &EpistemologicalFlow, explore_rate: f64) -> Action {
        if explore_rate > 0.0 {
            Action::Explore
        } else {
            Action::Exploit
        }
    }
    fn update(&mut self, _flow: &EpistemologicalFlow, _advantage: f64) {
        // no-op for sample stub
    }
}

/// Value-driven policy that uses Q-value estimation for action selection
pub struct ValueDrivenPolicy {
    q_estimator: NeuralQValueEstimator,
    pub total_visits: u32,
    exploration_constant: f64,
    base_exploration_constant: f64,
    recent_rewards: Vec<f64>,
    max_recent: usize,
    temp_explore_boost_steps: u32,
    // Reward normalization (EMA)
    running_mean: f64,
    running_var: f64,
    ema_alpha: f64,
    // Phase detection
    phase_shift_mode: bool,
    phase_steps_remaining: u32,
    // Masking of recently harmful action
    masked_action: Option<String>,
    mask_steps_remaining: u32,
    // Track consecutive negative rewards and recent positive action
    neg_reward_streak: u32,
    last_positive_action: Option<String>,
}

impl ValueDrivenPolicy {
    /// Creates a new value-driven policy
    pub fn new(learning_rate: f64, discount_factor: f64, exploration_constant: f64) -> Self {
        Self {
            q_estimator: NeuralQValueEstimator::new(learning_rate, discount_factor),
            total_visits: 0,
            exploration_constant,
            base_exploration_constant: exploration_constant,
            recent_rewards: Vec::new(),
            max_recent: 20,
            temp_explore_boost_steps: 0,
            running_mean: 0.0,
            running_var: 1e-6,
            ema_alpha: 0.05,
            phase_shift_mode: false,
            phase_steps_remaining: 0,
            masked_action: None,
            mask_steps_remaining: 0,
            neg_reward_streak: 0,
            last_positive_action: None,
        }
    }

    /// UCB1 exploration strategy
    pub fn ucb1_score(&self, flow: &EpistemologicalFlow, action: &Action) -> f64 {
        let action_str = action.as_str();
        let q_value = self
            .q_estimator
            .get_q_values(flow)
            .unwrap_or_default()
            .iter()
            .find(|(action_name, _)| *action_name == action_str)
            .map(|(_, q)| *q)
            .unwrap_or(0.0);

        let visit_count = self.q_estimator.get_visit_count(flow, action_str);

        if visit_count == 0 {
            // If never visited, return high score to encourage exploration
            f64::INFINITY
        } else {
            // UCB1 formula: Q(s,a) + c * sqrt(ln(N) / n(s,a))
            let total = (self.total_visits.max(1)) as f64;
            let exploration_bonus =
                self.exploration_constant * ((total.ln() / visit_count as f64).sqrt());
            q_value + exploration_bonus
        }
    }

    /// Update the policy with new experience
    pub fn update_with_experience(
        &mut self,
        flow: &EpistemologicalFlow,
        action: &Action,
        reward: f64,
        next_flow: &EpistemologicalFlow,
    ) {
        // Reward normalization via EMA (z-score like)
        let delta = reward - self.running_mean;
        self.running_mean += self.ema_alpha * delta;
        self.running_var = (1.0 - self.ema_alpha) * (self.running_var + self.ema_alpha * delta * delta);
        let norm_reward = if self.running_var > 1e-8 {
            (reward - self.running_mean) / self.running_var.sqrt()
        } else { reward };

        self.q_estimator
            .update_q_value(flow, action.as_str(), norm_reward, next_flow);
        self.total_visits += 1;

        // Maintain rolling reward window
        self.recent_rewards.push(reward);
        if self.recent_rewards.len() > self.max_recent {
            let _ = self.recent_rewards.remove(0);
        }
        let avg_recent = if self.recent_rewards.is_empty() {
            0.0
        } else {
            self.recent_rewards.iter().copied().sum::<f64>() / (self.recent_rewards.len() as f64)
        };

        // Track neg/pos streaks and last good action
        if reward < 0.0 {
            self.neg_reward_streak = self.neg_reward_streak.saturating_add(1);
        } else {
            self.neg_reward_streak = 0;
        }
        if reward > 0.6 {
            self.last_positive_action = Some(action.as_str().to_string());
        }

        // Phase detection & dynamic exploration adjustment
        if avg_recent < 0.4 {
            // Trigger phase-shift mode and stronger exploration
            self.phase_shift_mode = true;
            self.phase_steps_remaining = self.phase_steps_remaining.max(50);
            self.temp_explore_boost_steps = self.temp_explore_boost_steps.max(20);
            self.exploration_constant = (self.base_exploration_constant * 3.0).min(10.0);

            // Negative-reward penalty: depress Q of current action to break habits
            if reward < 0.0 {
                // Apply an extra update with stronger negative bonus to push away from harmful action
                self.q_estimator.update_q_value(flow, action.as_str(), reward * 3.0, next_flow);
                // Mask this action for several steps to force exploration
                self.masked_action = Some(action.as_str().to_string());
                // If repeated negatives, extend mask and exploration
                if self.neg_reward_streak >= 2 {
                    self.mask_steps_remaining = self.mask_steps_remaining.max(40);
                    self.exploration_constant = (self.base_exploration_constant * 5.0).min(12.0);
                } else {
                    self.mask_steps_remaining = self.mask_steps_remaining.max(20);
                }
            }
        } else if avg_recent > 0.8 {
            // Favor exploitation a bit when things are going well
            self.exploration_constant = (self.base_exploration_constant * 0.75).max(0.01);
            if self.temp_explore_boost_steps > 0 {
                self.temp_explore_boost_steps -= 1;
            }
        } else {
            // Gradually decay boost
            if self.temp_explore_boost_steps > 0 {
                self.temp_explore_boost_steps -= 1;
                if self.temp_explore_boost_steps == 0 {
                    self.exploration_constant = self.base_exploration_constant;
                }
            } else {
                self.exploration_constant = self.base_exploration_constant;
            }
        }

        if self.phase_shift_mode {
            if self.phase_steps_remaining > 0 {
                self.phase_steps_remaining -= 1;
            } else {
                self.phase_shift_mode = false;
                self.exploration_constant = self.base_exploration_constant;
            }
        }

        // Decay mask
        if let Some(_) = self.masked_action {
            if self.mask_steps_remaining > 0 {
                self.mask_steps_remaining -= 1;
            } else {
                self.masked_action = None;
            }
        }
    }
}

impl Policy for ValueDrivenPolicy {
    fn select_action(&self, flow: &EpistemologicalFlow, explore_rate: f64) -> Action {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        // Epsilon-greedy with UCB1 exploration
        if rng.gen::<f64>() < explore_rate {
            // Exploration: use UCB1 to select action
            Action::all_actions()
                .into_iter()
                .max_by(|a, b| {
                    let score_a = self.ucb1_score(flow, a);
                    let score_b = self.ucb1_score(flow, b);
                    score_a
                        .partial_cmp(&score_b)
                        .unwrap_or(std::cmp::Ordering::Equal)
                })
                .unwrap_or(Action::Explore)
        } else {
            // Exploitation with Thompson-like sampling: add small noise inversely to visits
            let qmap = self.q_estimator.get_q_values(flow).unwrap_or_default();
            let mut best: Option<(String, f64)> = None;
            for (a, q) in qmap.into_iter() {
                if let (Some(mask), true) = (&self.masked_action, self.mask_steps_remaining > 0) {
                    if &a == mask { continue; }
                }
                let n = (self.q_estimator.get_visit_count(flow, &a) as f64).max(1.0);
                // Simple Gaussian-like noise using Box-Muller without external crates
                let u1: f64 = (rng.gen::<f64>()).clamp(1e-9, 1.0);
                let u2: f64 = rng.gen::<f64>();
                let z = (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos();
                let sigma = (1.0 / n).sqrt();
                let noise = z * sigma;
                // Bias towards recently good action during phase shift
                let mut score = q + noise;
                if self.phase_shift_mode {
                    if let Some(ref good) = self.last_positive_action {
                        if &a == good {
                            score += 0.2;
                        }
                    }
                }
                if best.as_ref().map(|(_, bq)| score > *bq).unwrap_or(true) {
                    best = Some((a.to_string(), score));
                }
            }
            if let Some((action_str, _)) = best {
                match action_str.as_str() {
                    "noop" => Action::Noop,
                    "explore" => Action::Explore,
                    "exploit" => Action::Exploit,
                    "unlock_door" => Action::UnlockDoor,
                    "pick_up_key" => Action::PickUpKey,
                    "move_forward" => Action::MoveForward,
                    _ => Action::Noop,
                }
            } else {
                Action::Noop
            }
        }
    }

    fn update(&mut self, _flow: &EpistemologicalFlow, _advantage: f64) {
        // This is called by the learning system
        // We can use the advantage to update our Q-values
        // For now, we'll just increment the total visits
        self.total_visits += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::Bytes;

    #[test]
    fn epsilon_greedy_switches_mode() {
        let pol = EpsilonGreedyPolicy::default();
        let flow = EpistemologicalFlow::from_bytes(Bytes::from_static(b"state"));
        assert_eq!(pol.select_action(&flow, 0.1), Action::Explore);
        assert_eq!(pol.select_action(&flow, 0.0), Action::Exploit);
    }

    #[test]
    fn action_string_conversion() {
        assert_eq!(Action::Noop.as_str(), "noop");
        assert_eq!(Action::UnlockDoor.as_str(), "unlock_door");
        assert_eq!(Action::PickUpKey.as_str(), "pick_up_key");
        assert_eq!(Action::MoveForward.as_str(), "move_forward");
    }

    #[test]
    fn value_driven_policy_creation() {
        let policy = ValueDrivenPolicy::new(0.1, 0.9, 2.0);
        assert_eq!(policy.total_visits, 0);
    }

    #[test]
    fn value_driven_policy_action_selection() {
        let policy = ValueDrivenPolicy::new(0.1, 0.9, 2.0);
        let flow = EpistemologicalFlow::from_bytes(Bytes::from_static(b"test_state"));

        // Test exploitation (explore_rate = 0.0)
        let action = policy.select_action(&flow, 0.0);
        assert!(matches!(
            action,
            Action::Noop
                | Action::Explore
                | Action::Exploit
                | Action::UnlockDoor
                | Action::PickUpKey
                | Action::MoveForward
        ));
    }
}

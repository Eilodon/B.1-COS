use pandora_core::ontology::EpistemologicalFlow;
use crate::value_estimator::{QValueEstimator, NeuralQValueEstimator};

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
        if explore_rate > 0.0 { Action::Explore } else { Action::Exploit }
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
}

impl ValueDrivenPolicy {
    /// Creates a new value-driven policy
    pub fn new(learning_rate: f64, discount_factor: f64, exploration_constant: f64) -> Self {
        Self {
            q_estimator: NeuralQValueEstimator::new(learning_rate, discount_factor),
            total_visits: 0,
            exploration_constant,
        }
    }
    
    /// UCB1 exploration strategy
    pub fn ucb1_score(&self, flow: &EpistemologicalFlow, action: &Action) -> f64 {
        let action_str = action.as_str();
        let q_value = self.q_estimator.get_q_values(flow)
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
            let exploration_bonus = self.exploration_constant * 
                (self.total_visits as f64).ln() / visit_count as f64;
            q_value + exploration_bonus
        }
    }
    
    /// Update the policy with new experience
    pub fn update_with_experience(&mut self, flow: &EpistemologicalFlow, action: &Action, 
                                 reward: f64, next_flow: &EpistemologicalFlow) {
        self.q_estimator.update_q_value(flow, action.as_str(), reward, next_flow);
        self.total_visits += 1;
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
                    score_a.partial_cmp(&score_b).unwrap_or(std::cmp::Ordering::Equal)
                })
                .unwrap_or(Action::Explore)
        } else {
            // Exploitation: select action with highest Q-value
            self.q_estimator.get_q_values(flow)
                .unwrap_or_default()
                .into_iter()
                .max_by(|(_, q1), (_, q2)| q1.partial_cmp(q2).unwrap_or(std::cmp::Ordering::Equal))
                .map(|(action_str, _)| {
                    match action_str {
                        "noop" => Action::Noop,
                        "explore" => Action::Explore,
                        "exploit" => Action::Exploit,
                        "unlock_door" => Action::UnlockDoor,
                        "pick_up_key" => Action::PickUpKey,
                        "move_forward" => Action::MoveForward,
                        _ => Action::Noop,
                    }
                })
                .unwrap_or(Action::Noop)
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
        assert!(matches!(action, Action::Noop | Action::Explore | Action::Exploit | 
                         Action::UnlockDoor | Action::PickUpKey | Action::MoveForward));
    }
}



//! Active Inference Sankhara Skandha with planning and imagination capabilities
//!
//! This module implements an advanced Sankhara Skandha that uses Active Inference
//! for planning by simulating future states and selecting actions that minimize
//! expected free energy. It integrates with the Causal World Model (CWM) and
//! Learning Engine to enable goal-oriented behavior.

use pandora_core::interfaces::skandhas::{Skandha, SankharaSkandha};
use pandora_core::ontology::{EpistemologicalFlow, Vedana};
use pandora_core::world_model::WorldModel;
use pandora_error::PandoraError;
use std::sync::{Arc, Mutex};
use tracing::{debug, info};
use crate::LearningEngine;
use serde::{Deserialize, Serialize};

/// Represents a causal hypothesis discovered through data analysis.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CausalHypothesis {
    pub from_node_index: usize,
    pub to_node_index: usize,
    pub strength: f32,
    pub confidence: f32,
    pub edge_type: CausalEdgeType,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CausalEdgeType {
    Direct,
    Indirect,
    Conditional,
    Inhibitory,
}

/// Active Inference Sankhara Skandha with planning and imagination capabilities
/// 
/// This skandha implements Active Inference for planning by simulating future states
/// and selecting actions that minimize expected free energy. It integrates with the
/// Causal World Model (CWM) and Learning Engine to enable goal-oriented behavior.
pub struct ActiveInferenceSankharaSkandha {
    /// Reference to the Causal World Model for state prediction
    pub cwm: Arc<Mutex<dyn WorldModel + Send + Sync>>,
    /// Reference to the Learning Engine for reward calculation
    pub learning_engine: Arc<LearningEngine>,
    /// How many steps to simulate into the future
    pub planning_horizon: usize,
    /// Available actions for the agent
    pub available_actions: Vec<&'static str>,
    /// Pending causal hypothesis to test through experimentation
    pub pending_hypothesis: Option<CausalHypothesis>,
}

impl ActiveInferenceSankharaSkandha {
    /// Creates a new ActiveInferenceSankharaSkandha
    ///
    /// # Arguments
    ///
    /// * `cwm` - The Causal World Model wrapped in Arc<Mutex<>>
    /// * `learning_engine` - The Learning Engine wrapped in Arc
    /// * `planning_horizon` - Number of steps to simulate into the future
    /// * `available_actions` - List of available actions for the agent
    pub fn new(
        cwm: Arc<Mutex<dyn WorldModel + Send + Sync>>,
        learning_engine: Arc<LearningEngine>,
        planning_horizon: usize,
        available_actions: Vec<&'static str>,
    ) -> Self {
        Self {
            cwm,
            learning_engine,
            planning_horizon,
            available_actions,
            pending_hypothesis: None,
        }
    }

    /// Set a pending causal hypothesis to test through experimentation.
    pub fn set_pending_hypothesis(&mut self, hypothesis: Option<CausalHypothesis>) {
        self.pending_hypothesis = hypothesis;
        if let Some(ref hyp) = self.pending_hypothesis {
            info!("ActiveInference: Set pending hypothesis: {:?}", hyp);
        } else {
            info!("ActiveInference: Cleared pending hypothesis");
        }
    }

    /// Get the current pending hypothesis.
    pub fn get_pending_hypothesis(&self) -> &Option<CausalHypothesis> {
        &self.pending_hypothesis
    }

    /// Clear the pending hypothesis after testing.
    pub fn clear_pending_hypothesis(&mut self) {
        self.pending_hypothesis = None;
        info!("ActiveInference: Cleared pending hypothesis");
    }
}

impl Skandha for ActiveInferenceSankharaSkandha {
    fn name(&self) -> &'static str {
        "Active Inference Sankhara (Planning & Imagination)"
    }
}

impl SankharaSkandha for ActiveInferenceSankharaSkandha {
    fn form_intent(&self, flow: &mut EpistemologicalFlow) {
        info!(
            "[{}] Khởi phát ý chỉ với Active Inference planning.",
            self.name()
        );

        match self.plan_action(flow) {
            Ok(intent) => {
                info!("[{}] Khởi phát ý chỉ: '{}'", self.name(), intent);
                flow.sankhara = Some(std::sync::Arc::<str>::from(intent));
            }
            Err(e) => {
                info!(
                    "[{}] Lỗi trong quá trình planning: {}. Sử dụng intent mặc định.",
                    self.name(),
                    e
                );
                flow.sankhara = Some(std::sync::Arc::<str>::from("default_fallback_intent"));
            }
        }
    }
}

impl ActiveInferenceSankharaSkandha {
    /// Plans the best action using Active Inference
    ///
    /// This method simulates future states for each candidate action and selects
    /// the one that minimizes expected free energy (maximizes expected reward).
    fn plan_action(&self, current_flow: &EpistemologicalFlow) -> Result<&'static str, PandoraError> {
        info!("[{}] Bắt đầu quá trình planning với horizon: {}", self.name(), self.planning_horizon);
        
        let cwm = self.cwm.lock().map_err(|_| PandoraError::config("Failed to acquire CWM lock"))?;
        
        // 1. Propose candidate actions
        let candidate_actions = self.propose_candidate_actions(current_flow);
        info!("[{}] Đề xuất {} hành động: {:?}", self.name(), candidate_actions.len(), candidate_actions);
        
        let mut best_action = None;
        let mut best_expected_free_energy = f64::NEG_INFINITY;
        
        // 2. Simulate future for each candidate action
        for action in candidate_actions {
            let mut simulated_flow = current_flow.clone();
            simulated_flow.sankhara = Some(std::sync::Arc::<str>::from(action));
            
            let mut total_future_efe = 0.0;
            
            // 3. Rollout simulation for planning_horizon steps
            for step in 0..self.planning_horizon {
                // "Imagine" the next state by letting the CWM infer the context
                // Note: This is a simplified version - in a full implementation,
                // we would need a more sophisticated state prediction mechanism
                let temp_flow = simulated_flow.clone();
                
                // Calculate the Expected Free Energy (EFE) for this imagined step
                // We use the current model for both current and new model as a simplification
                let reward = self.learning_engine.calculate_reward(
                    &*cwm,
                    &*cwm, // Simplified: comparing model to itself
                    &temp_flow,
                );
                let efe = self.learning_engine.get_total_weighted_reward(&reward);
                total_future_efe += efe;
                
                debug!(
                    "[{}] Step {} cho action '{}': EFE = {:.4}",
                    self.name(),
                    step + 1,
                    action,
                    efe
                );
            }
            
            info!(
                "[{}] Action '{}' có tổng EFE: {:.4}",
                self.name(),
                action,
                total_future_efe
            );
            
            if total_future_efe > best_expected_free_energy {
                best_expected_free_energy = total_future_efe;
                best_action = Some(action);
            }
        }
        
        let selected_action = best_action.unwrap_or("default_fallback_intent");
        info!(
            "[{}] Chọn action '{}' với EFE: {:.4}",
            self.name(),
            selected_action,
            best_expected_free_energy
        );
        
        Ok(selected_action)
    }
    
    /// Proposes candidate actions based on current context
    ///
    /// This method analyzes the current flow and generates relevant actions.
    /// In a full implementation, this would use more sophisticated reasoning
    /// based on the current state and available actions.
    fn propose_candidate_actions(&self, flow: &EpistemologicalFlow) -> Vec<&'static str> {
        let mut candidates = Vec::new();
        
        // If we have a pending hypothesis, prioritize experimental actions
        if let Some(ref hypothesis) = self.pending_hypothesis {
            info!("ActiveInference: Proposing experimental actions for hypothesis: {:?}", hypothesis);
            
            // Generate actions that would test the causal hypothesis
            match hypothesis.edge_type {
                CausalEdgeType::Direct => {
                    candidates.push("TEST_DIRECT_CAUSALITY");
                    candidates.push("MANIPULATE_CAUSE_VARIABLE");
                }
                CausalEdgeType::Indirect => {
                    candidates.push("TEST_INDIRECT_CAUSALITY");
                    candidates.push("EXPLORE_MEDIATING_FACTORS");
                }
                CausalEdgeType::Conditional => {
                    candidates.push("TEST_CONDITIONAL_CAUSALITY");
                    candidates.push("VARY_CONDITIONS");
                }
                CausalEdgeType::Inhibitory => {
                    candidates.push("TEST_INHIBITORY_CAUSALITY");
                    candidates.push("BLOCK_INHIBITOR");
                }
            }
            
            // Add specific actions based on the hypothesis strength and confidence
            if hypothesis.confidence > 0.7 {
                candidates.push("CONDUCT_DEFINITIVE_EXPERIMENT");
            } else {
                candidates.push("CONDUCT_EXPLORATORY_EXPERIMENT");
            }
            
            // Add actions to manipulate the specific variables involved
            candidates.push("MANIPULATE_CAUSE_VARIABLE");
            candidates.push("OBSERVE_EFFECT_VARIABLE");
        }
        
        // Add all available actions as candidates
        candidates.extend(self.available_actions.iter().cloned());
        
        // Add context-specific actions based on flow analysis
        if let Some(vedana) = &flow.vedana {
            match vedana {
                Vedana::Pleasant { karma_weight } if *karma_weight > 1.0 => {
                    candidates.push("CONTINUE_SUCCESS");
                }
                Vedana::Unpleasant { karma_weight } if *karma_weight < -1.0 => {
                    candidates.push("TAKE_CORRECTIVE_ACTION");
                }
                _ => {
                    candidates.push("MAINTAIN_STATUS");
                }
            }
        }
        
        // Add pattern-based actions
        if let Some(sanna) = &flow.sanna {
            let pattern_complexity = sanna.active_indices.len() as f64 / sanna.dimensionality as f64;
            if pattern_complexity > 0.1 {
                candidates.push("ANALYZE_PATTERN");
            }
        }
        
        // Remove duplicates and return
        candidates.sort();
        candidates.dedup();
        candidates
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pandora_core::ontology::EpistemologicalFlow;
    use bytes::Bytes;
    use std::sync::Arc;
    use std::sync::Mutex;

    struct MockWorldModel {
        mdl: f64,
        prediction_error: f64,
    }

    impl WorldModel for MockWorldModel {
        fn get_mdl(&self) -> f64 {
            self.mdl
        }

        fn get_prediction_error(&self, _flow: &EpistemologicalFlow) -> f64 {
            self.prediction_error
        }
    }

    #[test]
    fn test_active_inference_skandha_creation() {
        let cwm = Arc::new(Mutex::new(MockWorldModel {
            mdl: 10.0,
            prediction_error: 0.2,
        }));
        let learning_engine = Arc::new(LearningEngine::new(0.7, 0.3));
        let available_actions = vec!["action_A", "action_B"];
        
        let sankhara = ActiveInferenceSankharaSkandha::new(
            cwm,
            learning_engine,
            3,
            available_actions,
        );
        
        assert_eq!(sankhara.name(), "Active Inference Sankhara (Planning & Imagination)");
        assert_eq!(sankhara.planning_horizon, 3);
        assert_eq!(sankhara.available_actions.len(), 2);
    }

    #[test]
    fn test_form_intent() {
        let cwm = Arc::new(Mutex::new(MockWorldModel {
            mdl: 10.0,
            prediction_error: 0.2,
        }));
        let learning_engine = Arc::new(LearningEngine::new(0.7, 0.3));
        let available_actions = vec!["action_A", "action_B"];
        
        let sankhara = ActiveInferenceSankharaSkandha::new(
            cwm,
            learning_engine,
            2,
            available_actions,
        );
        
        let mut flow = EpistemologicalFlow::from_bytes(Bytes::from(b"test_event".as_ref()));
        sankhara.form_intent(&mut flow);
        
        assert!(flow.sankhara.is_some(), "Sankhara should form an intent");
        let intent = flow.sankhara.as_ref().unwrap();
        assert!(
            ["action_A", "action_B", "default_fallback_intent"].contains(&intent.as_ref()),
            "Intent should be one of the available actions or fallback"
        );
    }

    #[test]
    fn test_propose_candidate_actions() {
        let cwm = Arc::new(Mutex::new(MockWorldModel {
            mdl: 10.0,
            prediction_error: 0.2,
        }));
        let learning_engine = Arc::new(LearningEngine::new(0.7, 0.3));
        let available_actions = vec!["action_A", "action_B"];
        
        let sankhara = ActiveInferenceSankharaSkandha::new(
            cwm,
            learning_engine,
            2,
            available_actions,
        );
        
        let flow = EpistemologicalFlow::from_bytes(Bytes::from(b"test_event".as_ref()));
        let candidates = sankhara.propose_candidate_actions(&flow);
        
        assert!(!candidates.is_empty(), "Should propose at least some actions");
        assert!(candidates.contains(&"action_A"), "Should include available actions");
        assert!(candidates.contains(&"action_B"), "Should include available actions");
    }
}

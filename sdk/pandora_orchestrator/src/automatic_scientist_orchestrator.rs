//! Automatic Scientist Orchestrator
//!
//! This orchestrator implements the complete "Automatic Scientist" loop,
//! integrating MCG, CWM, and ActiveInferenceSankharaSkandha to enable
//! autonomous causal discovery and knowledge crystallization.

#[cfg(feature = "ml")]
use std::sync::{Arc, Mutex};
#[cfg(feature = "ml")]
use tracing::info;
#[cfg(feature = "ml")]
use pandora_core::ontology::EpistemologicalFlow;
#[cfg(feature = "ml")]
use pandora_core::world_model::DualIntrinsicReward;
#[cfg(feature = "ml")]
use pandora_core::interfaces::skandhas::SankharaSkandha;
#[cfg(feature = "ml")]
use pandora_cwm::model::InterdependentCausalModel;
#[cfg(feature = "ml")]
use pandora_learning_engine::{LearningEngine, ActiveInferenceSankharaSkandha};
#[cfg(feature = "ml")]
use pandora_learning_engine::active_inference_skandha::CausalHypothesis as LearningCausalHypothesis;
#[cfg(feature = "ml")]
use pandora_mcg::enhanced_mcg::{EnhancedMetaCognitiveGovernor, ActionTrigger};
#[cfg(feature = "ml")]
use pandora_error::PandoraError;

/// The Automatic Scientist Orchestrator coordinates the complete self-improvement loop.
///
/// This orchestrator manages the interaction between:
/// 1. Enhanced MCG for causal discovery
/// 2. CWM for world modeling and knowledge storage
/// 3. ActiveInferenceSankharaSkandha for experimental planning
/// 4. Learning Engine for reward calculation
#[cfg(feature = "ml")]
pub struct AutomaticScientistOrchestrator {
    /// The Causal World Model for storing and reasoning about causal relationships
    cwm: Arc<Mutex<InterdependentCausalModel>>,
    /// The Learning Engine for calculating rewards and learning
    learning_engine: Arc<LearningEngine>,
    /// The Active Inference Sankhara Skandha for planning experiments
    sankhara: Arc<Mutex<ActiveInferenceSankharaSkandha>>,
    /// The Enhanced Meta-Cognitive Governor for causal discovery
    mcg: Arc<Mutex<EnhancedMetaCognitiveGovernor>>,
    /// Current experiment state
    experiment_state: Arc<Mutex<ExperimentState>>,
}

/// Represents the current state of an ongoing experiment
#[derive(Debug, Clone)]
pub struct ExperimentState {
    /// Whether an experiment is currently active
    pub is_active: bool,
    /// The hypothesis being tested
    pub hypothesis: Option<pandora_mcg::causal_discovery::CausalHypothesis>,
    /// Number of experiment steps completed
    pub steps_completed: usize,
    /// Maximum number of steps for the experiment
    pub max_steps: usize,
    /// Results collected so far
    pub results: Vec<ExperimentResult>,
}

/// Represents the result of an experiment step
#[derive(Debug, Clone)]
pub struct ExperimentResult {
    pub step: usize,
    pub action_taken: String,
    pub observation: Vec<f32>,
    pub reward: f64,
    pub hypothesis_confirmed: bool,
}

#[cfg(feature = "ml")]
impl AutomaticScientistOrchestrator {
    /// Creates a new Automatic Scientist Orchestrator
    pub fn new(
        cwm: Arc<Mutex<InterdependentCausalModel>>,
        learning_engine: Arc<LearningEngine>,
        sankhara: Arc<Mutex<ActiveInferenceSankharaSkandha>>,
        mcg: Arc<Mutex<EnhancedMetaCognitiveGovernor>>,
    ) -> Self {
        Self {
            cwm,
            learning_engine,
            sankhara,
            mcg,
            experiment_state: Arc::new(Mutex::new(ExperimentState {
                is_active: false,
                hypothesis: None,
                steps_completed: 0,
                max_steps: 10,
                results: Vec::new(),
            })),
        }
    }

    /// Runs one cycle of the Automatic Scientist loop
    ///
    /// This method implements the complete self-improvement cycle:
    /// 1. MCG monitors and discovers causal hypotheses
    /// 2. If a hypothesis is found, enter experiment mode
    /// 3. Sankhara plans and executes experiments
    /// 4. Results are analyzed and knowledge is crystallized
    pub async fn run_cycle(&self, current_flow: &mut EpistemologicalFlow) -> Result<(), PandoraError> {
        info!("=== Automatic Scientist Cycle Start ===");

        // 1. MCG monitors and decides on meta-actions
        let action_trigger = {
            let mut mcg = self.mcg.lock().map_err(|_| PandoraError::config("Failed to acquire MCG lock"))?;
            let cwm = self.cwm.lock().map_err(|_| PandoraError::config("Failed to acquire CWM lock"))?;
            
            // Calculate current reward
            let reward = self.learning_engine.calculate_reward(&*cwm, &*cwm, current_flow);
            let dual_reward = DualIntrinsicReward {
                compression_reward: self.learning_engine.get_total_weighted_reward(&reward),
                prediction_reward: 0.5, // Placeholder
            };
            
            mcg.monitor_and_decide(&dual_reward, &*cwm as &dyn std::any::Any)
        };

        // 2. Handle the action trigger
        match action_trigger {
            ActionTrigger::ProposeCausalHypothesis { hypothesis } => {
                info!("MCG proposed causal hypothesis: {:?}", hypothesis);
                self.start_experiment(hypothesis).await?;
            }
            ActionTrigger::NoAction => {
                info!("MCG: No action required");
            }
            _ => {
                info!("MCG: Other action triggered: {:?}", action_trigger);
            }
        }

        // 3. If an experiment is active, run one step
        if self.is_experiment_active().await? {
            self.run_experiment_step(current_flow).await?;
        }

        info!("=== Automatic Scientist Cycle Complete ===");
        Ok(())
    }

    /// Starts a new experiment with the given hypothesis
    async fn start_experiment(&self, hypothesis: pandora_mcg::causal_discovery::CausalHypothesis) -> Result<(), PandoraError> {
        info!("Starting experiment for hypothesis: {:?}", hypothesis);
        
        // Set experiment state
        {
            let mut state = self.experiment_state.lock().map_err(|_| PandoraError::config("Failed to acquire experiment state lock"))?;
            state.is_active = true;
            state.hypothesis = Some(hypothesis.clone());
            state.steps_completed = 0;
            state.results.clear();
        }

        // Set the hypothesis in the Sankhara Skandha
        {
            let mut sankhara = self.sankhara.lock().map_err(|_| PandoraError::config("Failed to acquire Sankhara lock"))?;
            // Convert MCG hypothesis to Learning hypothesis
            let learning_hypothesis = LearningCausalHypothesis {
                from_node_index: hypothesis.from_node_index,
                to_node_index: hypothesis.to_node_index,
                strength: hypothesis.strength,
                confidence: hypothesis.confidence,
                edge_type: match hypothesis.edge_type {
                    pandora_mcg::causal_discovery::CausalEdgeType::Direct => pandora_learning_engine::active_inference_skandha::CausalEdgeType::Direct,
                    pandora_mcg::causal_discovery::CausalEdgeType::Indirect => pandora_learning_engine::active_inference_skandha::CausalEdgeType::Indirect,
                    pandora_mcg::causal_discovery::CausalEdgeType::Conditional => pandora_learning_engine::active_inference_skandha::CausalEdgeType::Conditional,
                    pandora_mcg::causal_discovery::CausalEdgeType::Inhibitory => pandora_learning_engine::active_inference_skandha::CausalEdgeType::Inhibitory,
                },
            };
            sankhara.set_pending_hypothesis(Some(learning_hypothesis));
        }

        info!("Experiment started successfully");
        Ok(())
    }

    /// Runs one step of the current experiment
    async fn run_experiment_step(&self, flow: &mut EpistemologicalFlow) -> Result<(), PandoraError> {
        let mut state = self.experiment_state.lock().map_err(|_| PandoraError::config("Failed to acquire experiment state lock"))?;
        
        if !state.is_active {
            return Ok(());
        }

        if state.steps_completed >= state.max_steps {
            info!("Experiment completed after {} steps", state.steps_completed);
            self.complete_experiment().await?;
            return Ok(());
        }

        // Let the Sankhara Skandha form an intent (plan an action)
        {
            let sankhara = self.sankhara.lock().map_err(|_| PandoraError::config("Failed to acquire Sankhara lock"))?;
            sankhara.form_intent(flow);
        }

        // Execute the planned action (simplified)
        let action = flow.sankhara.as_ref().map(|s| s.as_ref()).unwrap_or("default_action");
        info!("Executing experimental action: {}", action);

        // Simulate the effect of the action
        let observation = self.simulate_action_effect(action).await?;
        
        // Calculate reward
        let cwm = self.cwm.lock().map_err(|_| PandoraError::config("Failed to acquire CWM lock"))?;
        let reward = self.learning_engine.calculate_reward(&*cwm, &*cwm, flow);
        let reward_value = self.learning_engine.get_total_weighted_reward(&reward);

        // Check if hypothesis is confirmed
        let hypothesis_confirmed = self.check_hypothesis_confirmation(&observation).await?;

        // Record the result
        let step = state.steps_completed;
        state.results.push(ExperimentResult {
            step,
            action_taken: action.to_string(),
            observation,
            reward: reward_value,
            hypothesis_confirmed,
        });

        state.steps_completed += 1;

        info!("Experiment step {} completed. Hypothesis confirmed: {}", 
              state.steps_completed, hypothesis_confirmed);

        Ok(())
    }

    /// Simulates the effect of an action (placeholder implementation)
    async fn simulate_action_effect(&self, action: &str) -> Result<Vec<f32>, PandoraError> {
        // In a real implementation, this would interact with the environment
        // For now, we'll simulate based on the action type
        let mut observation = vec![0.0; 64];
        
        match action {
            "MANIPULATE_CAUSE_VARIABLE" | "ACTIVATE_CAUSE_NODE" => {
                observation[0] = 1.0; // Simulate cause activation
            }
            "OBSERVE_EFFECT_VARIABLE" | "MEASURE_EFFECT_MAGNITUDE" => {
                observation[1] = 0.8; // Simulate effect observation
            }
            "CONDUCT_DEFINITIVE_EXPERIMENT" => {
                observation[0] = 1.0;
                observation[1] = 0.9; // High confidence result
            }
            _ => {
                observation[2] = 0.5; // Generic observation
            }
        }
        
        Ok(observation)
    }

    /// Checks if the current hypothesis is confirmed by the observations
    async fn check_hypothesis_confirmation(&self, observation: &[f32]) -> Result<bool, PandoraError> {
        // Simplified hypothesis confirmation logic
        // In a real implementation, this would be more sophisticated
        
        let state = self.experiment_state.lock().map_err(|_| PandoraError::config("Failed to acquire experiment state lock"))?;
        
        if let Some(ref hypothesis) = state.hypothesis {
            // Simple heuristic: if we see both cause and effect activation
            let cause_activated = observation.get(0).map(|&x| x > 0.5).unwrap_or(false);
            let effect_observed = observation.get(1).map(|&x| x > 0.5).unwrap_or(false);
            
            // For direct causality, both should be present
            if hypothesis.edge_type == pandora_mcg::causal_discovery::CausalEdgeType::Direct {
                return Ok(cause_activated && effect_observed);
            }
            
            // For other types, just check if we have some evidence
            return Ok(cause_activated || effect_observed);
        }
        
        Ok(false)
    }

    /// Completes the current experiment and crystallizes knowledge if confirmed
    async fn complete_experiment(&self) -> Result<(), PandoraError> {
        info!("Completing experiment...");
        
        let (hypothesis, results) = {
            let state = self.experiment_state.lock().map_err(|_| PandoraError::config("Failed to acquire experiment state lock"))?;
            (state.hypothesis.clone(), state.results.clone())
        };

        if let Some(hypothesis) = hypothesis {
            // Analyze results to determine if hypothesis is confirmed
            let confirmed_steps = results.iter().filter(|r| r.hypothesis_confirmed).count();
            let confirmation_rate = confirmed_steps as f32 / results.len() as f32;
            
            info!("Experiment results: {}/{} steps confirmed hypothesis ({}%)", 
                  confirmed_steps, results.len(), (confirmation_rate * 100.0) as u32);

            // If hypothesis is sufficiently confirmed, crystallize it
            if confirmation_rate > 0.6 { // 60% confirmation threshold
                info!("Hypothesis confirmed! Crystallizing knowledge...");
                
                // Convert MCG hypothesis to CWM hypothesis
                #[cfg(feature = "ml")]
                let cwm_hypothesis = pandora_cwm::model::CausalHypothesis {
                    from_node_index: hypothesis.from_node_index,
                    to_node_index: hypothesis.to_node_index,
                    strength: hypothesis.strength,
                    confidence: hypothesis.confidence,
                    edge_type: match hypothesis.edge_type {
                        pandora_mcg::causal_discovery::CausalEdgeType::Direct => {
                            #[cfg(feature = "ml")]
                            pandora_cwm::model::CausalEdgeType::Direct
                        },
                        pandora_mcg::causal_discovery::CausalEdgeType::Indirect => {
                            #[cfg(feature = "ml")]
                            pandora_cwm::model::CausalEdgeType::Indirect
                        },
                        pandora_mcg::causal_discovery::CausalEdgeType::Conditional => {
                            #[cfg(feature = "ml")]
                            pandora_cwm::model::CausalEdgeType::Conditional
                        },
                        pandora_mcg::causal_discovery::CausalEdgeType::Inhibitory => {
                            #[cfg(feature = "ml")]
                            pandora_cwm::model::CausalEdgeType::Inhibitory
                        },
                    },
                };

                // Crystallize the causal link in the CWM
                #[cfg(feature = "ml")]
                {
                    let mut cwm = self.cwm.lock().map_err(|_| PandoraError::config("Failed to acquire CWM lock"))?;
                    cwm.crystallize_causal_link(&cwm_hypothesis)?;
                }

                info!("Knowledge crystallized successfully!");
            } else {
                info!("Hypothesis not sufficiently confirmed. Discarding.");
            }
        }

        // Reset experiment state
        {
            let mut state = self.experiment_state.lock().map_err(|_| PandoraError::config("Failed to acquire experiment state lock"))?;
            state.is_active = false;
            state.hypothesis = None;
            state.steps_completed = 0;
            state.results.clear();
        }

        // Clear hypothesis from Sankhara
        {
            let mut sankhara = self.sankhara.lock().map_err(|_| PandoraError::config("Failed to acquire Sankhara lock"))?;
            sankhara.clear_pending_hypothesis();
        }

        info!("Experiment completed and cleaned up");
        Ok(())
    }

    /// Checks if an experiment is currently active
    async fn is_experiment_active(&self) -> Result<bool, PandoraError> {
        let state = self.experiment_state.lock().map_err(|_| PandoraError::config("Failed to acquire experiment state lock"))?;
        Ok(state.is_active)
    }

    /// Gets the current experiment state
    pub fn get_experiment_state(&self) -> Result<ExperimentState, PandoraError> {
        let state = self.experiment_state.lock().map_err(|_| PandoraError::config("Failed to acquire experiment state lock"))?;
        Ok(state.clone())
    }
}

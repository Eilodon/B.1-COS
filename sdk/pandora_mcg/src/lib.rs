#[cfg(feature = "ml")]
use pandora_cwm::nn::uq_models::ProbabilisticOutput;
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActionTrigger {
    TriggerSelfImprovementLevel1 { reason: String, target_component: String },
    TriggerSelfImprovementLevel2 { reason: String, target_component: String },
    RequestMoreInformation { reason: String },
    NoAction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetaRule {
    #[cfg(feature = "ml")]
    IfUncertaintyExceeds { threshold: f32, action: ActionTrigger },
    IfCompressionRewardExceeds { threshold: f64, action: ActionTrigger },
}


pub struct RuleEngine {
    rules: Vec<MetaRule>,
}

impl RuleEngine {
    pub fn new(rules: Vec<MetaRule>) -> Self { Self { rules } }

    #[cfg(feature = "ml")]
    pub fn evaluate_ml(&self, output: &ProbabilisticOutput) -> ActionTrigger {
        let mean_variance = output.variance.mean_all().and_then(|t| t.to_scalar::<f32>());
        if let Ok(variance_val) = mean_variance {
            for rule in &self.rules {
                match rule {
                    MetaRule::IfUncertaintyExceeds { threshold, action } => {
                        if variance_val > *threshold {
                            info!(
                                "MCG: Phát hiện độ bất định ({:.4}) > ngưỡng ({:.4}). Kích hoạt hành động.",
                                variance_val, threshold
                            );
                            return action.clone();
                        }
                    }
                    _ => {} // Bỏ qua các quy tắc khác
                }
            }
        }
        info!("MCG: Trạng thái ổn định. Không cần hành động.");
        ActionTrigger::NoAction
    }

    pub fn evaluate(&self, reward: &pandora_core::world_model::DualIntrinsicReward) -> ActionTrigger {
        for rule in &self.rules {
            match rule {
                MetaRule::IfCompressionRewardExceeds { threshold, action } => {
                    if reward.compression_reward > *threshold {
                        info!(
                            "MCG: Phát hiện Phần thưởng Nén ({:.4}) > ngưỡng ({:.4}). Kích hoạt hành động!",
                            reward.compression_reward, threshold
                        );
                        return action.clone();
                    }
                }
                #[cfg(feature = "ml")]
                MetaRule::IfUncertaintyExceeds { .. } => {
                    // Bỏ qua quy tắc ML trong chế độ này
                }
            }
        }
        info!("MCG: Trạng thái ổn định. Không cần hành động.");
        ActionTrigger::NoAction
    }
}

pub struct MetaCognitiveGovernor { rule_engine: RuleEngine }

impl MetaCognitiveGovernor {
    pub fn new(rule_engine: RuleEngine) -> Self { Self { rule_engine } }

    #[cfg(feature = "ml")]
    pub fn monitor_and_decide_ml(&self, cwm_output: &ProbabilisticOutput) -> ActionTrigger {
        info!("\n--- Vòng lặp Siêu Nhận thức Bắt đầu ---");
        let decision = self.rule_engine.evaluate_ml(cwm_output);
        info!("--- Vòng lặp Siêu Nhận thức Kết thúc ---");
        decision
    }

    pub fn monitor_and_decide(&self, reward: &pandora_core::world_model::DualIntrinsicReward) -> ActionTrigger {
        info!("\n--- Vòng lặp Siêu Nhận thức Bắt đầu ---");
        let decision = self.rule_engine.evaluate(reward);
        info!("--- Vòng lặp Siêu Nhận thức Kết thúc ---");
        decision
    }
}

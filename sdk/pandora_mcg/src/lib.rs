#[cfg(feature = "ml")]
use pandora_cwm::nn::uq_models::ProbabilisticOutput;
use serde::{Deserialize, Serialize};

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
}

pub struct RuleEngine {
    rules: Vec<MetaRule>,
}

impl RuleEngine {
    pub fn new(rules: Vec<MetaRule>) -> Self { Self { rules } }

    #[cfg(feature = "ml")]
    pub fn evaluate(&self, output: &ProbabilisticOutput) -> ActionTrigger {
        let mean_variance = output.variance.mean_all().and_then(|t| t.to_scalar::<f32>());
        if let Ok(variance_val) = mean_variance {
            for rule in &self.rules {
                match rule {
                    MetaRule::IfUncertaintyExceeds { threshold, action } => {
                        if variance_val > *threshold {
                            println!(
                                "MCG: Phát hiện độ bất định ({:.4}) > ngưỡng ({:.4}). Kích hoạt hành động.",
                                variance_val, threshold
                            );
                            return action.clone();
                        }
                    }
                }
            }
        }
        println!("MCG: Trạng thái ổn định. Không cần hành động.");
        ActionTrigger::NoAction
    }
}

pub struct MetaCognitiveGovernor { rule_engine: RuleEngine }

impl MetaCognitiveGovernor {
    pub fn new(rule_engine: RuleEngine) -> Self { Self { rule_engine } }

    #[cfg(feature = "ml")]
    pub fn monitor_and_decide(&self, cwm_output: &ProbabilisticOutput) -> ActionTrigger {
        println!("\n--- Vòng lặp Siêu Nhận thức Bắt đầu ---");
        let decision = self.rule_engine.evaluate(cwm_output);
        println!("--- Vòng lặp Siêu Nhận thức Kết thúc ---");
        decision
    }
}

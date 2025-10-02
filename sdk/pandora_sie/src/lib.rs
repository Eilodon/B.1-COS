use async_trait::async_trait;
use pandora_error::PandoraError;
use pandora_mcg::ActionTrigger;
use tracing::{info, instrument};
#[cfg(feature = "metrics_instrumentation")]
use metrics::{counter, histogram};

pub struct ImprovementAction {
    pub description: String,
}

#[async_trait]
pub trait ImprovementStrategy: Send + Sync {
    fn name(&self) -> &'static str;
    async fn propose_action(
        &self,
        trigger: &ActionTrigger,
    ) -> Result<ImprovementAction, PandoraError>;
}

pub struct RefinementStrategy;

#[async_trait]
impl ImprovementStrategy for RefinementStrategy {
    fn name(&self) -> &'static str {
        "Level 1: Refinement Strategy"
    }

    async fn propose_action(
        &self,
        trigger: &ActionTrigger,
    ) -> Result<ImprovementAction, PandoraError> {
        match trigger {
            ActionTrigger::TriggerSelfImprovementLevel1 {
                reason,
                target_component,
            } => {
                info!(
                    "SIE ({}): Nhận yêu cầu tinh chỉnh cho '{}' vì '{}'",
                    self.name(),
                    target_component,
                    reason
                );
                let description = format!(
                    "Đề xuất: Chạy lại thành phần '{}' với các tham số dự phòng.",
                    target_component
                );
                Ok(ImprovementAction { description })
            }
            _ => Err(PandoraError::config(
                "Chiến lược không phù hợp với trigger này.",
            )),
        }
    }
}

pub struct ArchitectureSearchStrategy;

#[async_trait]
impl ImprovementStrategy for ArchitectureSearchStrategy {
    fn name(&self) -> &'static str { "Level 2: Architecture Search" }
    async fn propose_action(&self, trigger: &ActionTrigger) -> Result<ImprovementAction, PandoraError> {
        match trigger {
            ActionTrigger::TriggerSelfImprovementLevel2 { reason, target_component } => {
                info!("SIE ({}): Kiến trúc - '{}' vì '{}'", self.name(), target_component, reason);
                Ok(ImprovementAction { description: format!("Đề xuất: Tìm kiếm kiến trúc cho '{}'.", target_component) })
            }
            _ => Err(PandoraError::config("Chiến lược không phù hợp với trigger này.")),
        }
    }
}

pub struct CodeGenerationStrategy;

#[async_trait]
impl ImprovementStrategy for CodeGenerationStrategy {
    fn name(&self) -> &'static str { "Level 3: Code Generation" }
    async fn propose_action(&self, trigger: &ActionTrigger) -> Result<ImprovementAction, PandoraError> {
        match trigger {
            ActionTrigger::TriggerSelfImprovementLevel3 { reason, target_component } => {
                info!("SIE ({}): Sinh mã - '{}' vì '{}'", self.name(), target_component, reason);
                Ok(ImprovementAction { description: format!("Đề xuất: Sinh mã cải thiện cho '{}'.", target_component) })
            }
            _ => Err(PandoraError::config("Chiến lược không phù hợp với trigger này.")),
        }
    }
}

pub struct MetaLearningStrategy;

#[async_trait]
impl ImprovementStrategy for MetaLearningStrategy {
    fn name(&self) -> &'static str { "Level 4: Meta-Learning" }
    async fn propose_action(&self, trigger: &ActionTrigger) -> Result<ImprovementAction, PandoraError> {
        match trigger {
            ActionTrigger::TriggerSelfImprovementLevel4 { reason, target_component } => {
                info!("SIE ({}): Meta-learning - '{}' vì '{}'", self.name(), target_component, reason);
                Ok(ImprovementAction { description: format!("Đề xuất: Điều chỉnh siêu tham số/meta-policy cho '{}'.", target_component) })
            }
            _ => Err(PandoraError::config("Chiến lược không phù hợp với trigger này.")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn level1_refinement_works() {
        let sie = SelfImprovementEngine::new();
        let trig = ActionTrigger::TriggerSelfImprovementLevel1 { reason: "noise".into(), target_component: "orchestrator".into() };
        let action = sie.execute(&trig).await.unwrap();
        assert!(action.description.contains("Đề xuất"));
    }

    #[tokio::test]
    async fn level2_arch_search_works() {
        let sie = SelfImprovementEngine::new();
        let trig = ActionTrigger::TriggerSelfImprovementLevel2 { reason: "bottleneck".into(), target_component: "world_model".into() };
        let action = sie.execute(&trig).await.unwrap();
        assert!(action.description.contains("Tìm kiếm kiến trúc"));
    }

    #[tokio::test]
    async fn level3_codegen_works() {
        let sie = SelfImprovementEngine::new();
        let trig = ActionTrigger::TriggerSelfImprovementLevel3 { reason: "bug".into(), target_component: "tools".into() };
        let action = sie.execute(&trig).await.unwrap();
        assert!(action.description.contains("Sinh mã"));
    }

    #[tokio::test]
    async fn level4_meta_learning_works() {
        let sie = SelfImprovementEngine::new();
        let trig = ActionTrigger::TriggerSelfImprovementLevel4 { reason: "non-stationary".into(), target_component: "policy".into() };
        let action = sie.execute(&trig).await.unwrap();
        assert!(action.description.contains("siêu tham số"));
    }
}

/// Self-Improvement Engine that executes various improvement strategies.
///
/// This engine coordinates different levels of self-improvement actions based on
/// triggers from the Meta-Cognitive Governor. It supports four levels of improvement
/// from simple refinement to meta-learning.
///
/// # Examples
///
/// ```rust
/// use pandora_sie::SelfImprovementEngine;
/// use pandora_mcg::ActionTrigger;
///
/// # async fn example() {
/// let sie = SelfImprovementEngine::new();
/// let trigger = ActionTrigger::TriggerSelfImprovementLevel1 {
///     reason: "performance degradation".to_string(),
///     target_component: "world_model".to_string(),
/// };
///
/// let action = sie.execute(&trigger).await.unwrap();
/// assert!(action.description.contains("Đề xuất"));
/// # }
/// ```
pub struct SelfImprovementEngine {
    strategies: fnv::FnvHashMap<u8, Box<dyn ImprovementStrategy>>,
}

impl SelfImprovementEngine {
    pub fn new() -> Self {
        let mut strategies: fnv::FnvHashMap<u8, Box<dyn ImprovementStrategy>> =
            fnv::FnvHashMap::default();
        strategies.insert(1, Box::new(RefinementStrategy));
        strategies.insert(2, Box::new(ArchitectureSearchStrategy));
        strategies.insert(3, Box::new(CodeGenerationStrategy));
        strategies.insert(4, Box::new(MetaLearningStrategy));
        info!("SIE: Đã khởi tạo và đăng ký các chiến lược tự cải thiện.");
        Self { strategies }
    }

    #[instrument(skip(self))]
    pub async fn execute(
        &self,
        trigger: &ActionTrigger,
    ) -> Result<ImprovementAction, PandoraError> {
        info!("\n--- Động cơ Tự Cải thiện Bắt đầu ---");
        let strategy = match trigger {
            ActionTrigger::TriggerSelfImprovementLevel1 { .. } => self.strategies.get(&1),
            ActionTrigger::TriggerSelfImprovementLevel2 { .. } => self.strategies.get(&2),
            ActionTrigger::TriggerSelfImprovementLevel3 { .. } => self.strategies.get(&3),
            ActionTrigger::TriggerSelfImprovementLevel4 { .. } => self.strategies.get(&4),
            _ => None,
        };

        if let Some(s) = strategy {
            let start = std::time::Instant::now();
            let action = s.propose_action(trigger).await?;
            let _elapsed = start.elapsed().as_secs_f64();
            #[cfg(feature = "metrics_instrumentation")]
            {
                counter!("sie.proposals_total", 1);
                histogram!("sie.proposal_latency_seconds", _elapsed);
            }
            info!("SIE: Đã đề xuất hành động: '{}'", action.description);
            info!("--- Động cơ Tự Cải thiện Kết thúc ---");
            Ok(action)
        } else {
            Err(PandoraError::config(format!(
                "Không tìm thấy chiến lược phù hợp cho trigger: {:?}",
                trigger
            )))
        }
    }
}

impl Default for SelfImprovementEngine {
    fn default() -> Self {
        Self::new()
    }
}

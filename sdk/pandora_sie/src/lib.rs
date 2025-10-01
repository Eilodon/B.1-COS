use async_trait::async_trait;
use pandora_error::PandoraError;
use pandora_mcg::ActionTrigger;
use tracing::info;

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
            _ => Err(PandoraError::Config(
                "Chiến lược không phù hợp với trigger này.".to_string(),
            )),
        }
    }
}

pub struct SelfImprovementEngine {
    strategies: std::collections::HashMap<u8, Box<dyn ImprovementStrategy>>,
}

impl SelfImprovementEngine {
    pub fn new() -> Self {
        let mut strategies: std::collections::HashMap<u8, Box<dyn ImprovementStrategy>> =
            std::collections::HashMap::new();
        strategies.insert(1, Box::new(RefinementStrategy));
        info!("SIE: Đã khởi tạo và đăng ký các chiến lược tự cải thiện.");
        Self { strategies }
    }

    pub async fn execute(
        &self,
        trigger: &ActionTrigger,
    ) -> Result<ImprovementAction, PandoraError> {
        info!("\n--- Động cơ Tự Cải thiện Bắt đầu ---");
        let strategy = match trigger {
            ActionTrigger::TriggerSelfImprovementLevel1 { .. } => self.strategies.get(&1),
            ActionTrigger::TriggerSelfImprovementLevel2 { .. } => self.strategies.get(&2),
            _ => None,
        };

        if let Some(s) = strategy {
            let action = s.propose_action(trigger).await?;
            info!("SIE: Đã đề xuất hành động: '{}'", action.description);
            info!("--- Động cơ Tự Cải thiện Kết thúc ---");
            Ok(action)
        } else {
            Err(PandoraError::Config(format!(
                "Không tìm thấy chiến lược phù hợp cho trigger: {:?}",
                trigger
            )))
        }
    }
}

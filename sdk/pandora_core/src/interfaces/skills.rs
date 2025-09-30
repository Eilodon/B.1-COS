use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use serde_json::Value as SkillInput; // Đổi tên để rõ ràng
use pandora_error::PandoraError;

pub type SkillOutput = Result<serde_json::Value, PandoraError>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SkillDescriptor {
    pub name: String,
    pub description: String,
    pub input_schema: String,
    pub output_schema: String,
}

#[async_trait]
pub trait SkillModule {
    fn descriptor(&self) -> SkillDescriptor;
    async fn execute(&self, input: SkillInput) -> SkillOutput;
}

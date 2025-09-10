use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use serde_json::Value as SkillInput; // Đổi tên để rõ ràng

pub type SkillOutput = Result<serde_json::Value, String>;

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

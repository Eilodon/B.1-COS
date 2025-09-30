use pandora_core::interfaces::skills::SkillModule;
use pandora_core::ontology::CognitiveFlow;
use pandora_error::PandoraError;
use std::collections::HashMap;
use std::sync::Arc;

/// `SkillRegistry` chịu trách nhiệm lưu trữ và quản lý tất cả các skill có sẵn.
/// Trong tương lai, nó sẽ có khả năng tự động khám phá các skill.
pub struct SkillRegistry {
    skills: HashMap<String, Arc<dyn SkillModule>>,
}

impl SkillRegistry {
    pub fn new() -> Self {
        Self { skills: HashMap::new() }
    }

    /// Đăng ký một skill mới vào registry.
    pub fn register(&mut self, skill: Arc<dyn SkillModule>) {
        let name = skill.descriptor().name;
        self.skills.insert(name, skill);
    }

    /// Tìm một skill dựa trên tên của nó.
    pub fn get_skill(&self, name: &str) -> Option<&Arc<dyn SkillModule>> {
        self.skills.get(name)
    }
}

/// `Orchestrator` là bộ não trung tâm, điều phối luồng nhận thức và thực thi các skill.
pub struct Orchestrator {
    registry: Arc<SkillRegistry>,
}

impl Orchestrator {
    pub fn new(registry: Arc<SkillRegistry>) -> Self {
        Self { registry }
    }

    /// Xử lý một yêu cầu nhận thức đơn giản bằng cách định tuyến đến skill phù hợp.
    pub async fn process_request(
        &self,
        skill_name: &str,
        input: serde_json::Value,
    ) -> Result<serde_json::Value, PandoraError> {
        println!("Orchestrator: Nhận yêu cầu cho skill '{}'", skill_name);

        let skill = self
            .registry
            .get_skill(skill_name)
            .ok_or_else(|| PandoraError::Config(format!("Không tìm thấy skill với tên '{}'", skill_name)))?;

        let output = skill.execute(input).await?;
        println!("Orchestrator: Skill '{}' đã thực thi thành công.", skill_name);
        Ok(output)
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

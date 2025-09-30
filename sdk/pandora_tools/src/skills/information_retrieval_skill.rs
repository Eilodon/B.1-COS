#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn test_valid_query() {
        let skill = InformationRetrievalSkill;
        let input =
            json!({"query": "test", "documents": ["test document", "another doc", "test again"]});
        let output = skill.execute(input).await.unwrap();
        assert_eq!(output, json!({"matches": ["test document", "test again"]}));
    }

    #[tokio::test]
    async fn test_query_is_empty() {
        let skill = InformationRetrievalSkill;
        let input = json!({"query": "", "documents": ["a", "b"]});
        let output = skill.execute(input).await.unwrap();
        assert_eq!(output, json!({"matches": ["a", "b"]}));
    }
}
use async_trait::async_trait;
use pandora_core::interfaces::skills::{SkillDescriptor, SkillModule, SkillOutput};
use pandora_error::PandoraError;
use serde_json::json;
use serde_json::Value as SkillInput;

pub struct InformationRetrievalSkill;

#[async_trait]
impl SkillModule for InformationRetrievalSkill {
    fn descriptor(&self) -> SkillDescriptor {
        SkillDescriptor {
			name: "information_retrieval".to_string(),
			description: "Tìm kiếm thông tin đơn giản trong một danh sách văn bản.".to_string(),
			input_schema: r#"{"type":"object","properties":{"query":{"type":"string"},"documents":{"type":"array","items":{"type":"string"}}},"required":["query","documents"]}"#.to_string(),
			output_schema: r#"{"type":"object","properties":{"matches":{"type":"array","items":{"type":"string"}}}}"#.to_string(),
		}
    }

    async fn execute(&self, input: SkillInput) -> SkillOutput {
        let query =
            input
                .get("query")
                .and_then(|v| v.as_str())
                .ok_or(PandoraError::SkillExecution {
                    skill_name: "information_retrieval".into(),
                    message: "Thiếu trường 'query'".into(),
                })?;
        let docs = input.get("documents").and_then(|v| v.as_array()).ok_or(
            PandoraError::SkillExecution {
                skill_name: "information_retrieval".into(),
                message: "Thiếu trường 'documents'".into(),
            },
        )?;
        let matches: Vec<_> = docs
            .iter()
            .filter_map(|d| d.as_str())
            .filter(|doc| doc.to_lowercase().contains(&query.to_lowercase()))
            .collect();
        Ok(json!({"matches": matches}))
    }
}

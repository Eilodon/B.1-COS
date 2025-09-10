#[cfg(test)]
mod tests {
	use super::*;
	use serde_json::json;

	#[tokio::test]
	async fn test_valid_query() {
		let skill = InformationRetrievalSkill;
		let input = json!({"query": "test", "documents": ["test document", "another doc", "test again"]});
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
use b_one_core_native::interfaces::skills::{SkillModule, SkillDescriptor, SkillOutput};
use serde_json::Value as SkillInput;
use async_trait::async_trait;
use serde_json::json;

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
		let query = input.get("query").and_then(|v| v.as_str()).ok_or("Thiếu trường 'query'")?;
		let docs = input.get("documents").and_then(|v| v.as_array()).ok_or("Thiếu trường 'documents'")?;
		let matches: Vec<_> = docs.iter()
			.filter_map(|d| d.as_str())
			.filter(|doc| doc.to_lowercase().contains(&query.to_lowercase()))
			.collect();
		Ok(json!({"matches": matches}))
	}
}

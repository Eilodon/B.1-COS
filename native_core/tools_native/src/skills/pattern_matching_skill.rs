#[cfg(test)]
mod tests {
	use super::*;
	use serde_json::json;

	#[tokio::test]
	async fn test_simple_pattern_match() {
		let skill = PatternMatchingSkill;
		let input = json!({"pattern": "a*b", "candidates": ["ab", "aab", "b", "acb"]});
		let output = skill.execute(input).await.unwrap();
		assert_eq!(output, json!({"matches": ["ab", "aab", "acb"]}));
	}

	#[tokio::test]
	async fn test_no_match() {
		let skill = PatternMatchingSkill;
		let input = json!({"pattern": "x*", "candidates": ["a", "b", "c"]});
		let output = skill.execute(input).await.unwrap();
		assert_eq!(output, json!({"matches": []}));
	}
}
use core_native::interfaces::skills::{SkillModule, SkillDescriptor, SkillOutput};
use serde_json::Value as SkillInput;
use async_trait::async_trait;
use serde_json::{json, Value};
use std::collections::HashMap;

pub struct PatternMatchingSkill;

#[async_trait]
impl SkillModule for PatternMatchingSkill {
	fn descriptor(&self) -> SkillDescriptor {
		SkillDescriptor {
			name: "pattern_matching".to_string(),
			description: "Khớp mẫu đơn giản giữa các chuỗi với wildcard '*' (dạng glob).".to_string(),
			input_schema: r#"{"type":"object","properties":{"pattern":{"type":"string"},"candidates":{"type":"array","items":{"type":"string"}}},"required":["pattern","candidates"]}"#.to_string(),
			output_schema: r#"{"type":"object","properties":{"matches":{"type":"array","items":{"type":"string"}}}}"#.to_string(),
		}
	}

	async fn execute(&self, input: SkillInput) -> SkillOutput {
		let pattern = input.get("pattern").and_then(|v| v.as_str()).ok_or("Thiếu trường 'pattern'")?;
		let candidates = input.get("candidates").and_then(|v| v.as_array()).ok_or("Thiếu trường 'candidates'")?;
		let pat = pattern.replace("*", ".*");
		let re = regex::Regex::new(&format!("^{}$", pat)).map_err(|e| format!("Lỗi regex: {}", e))?;
		let matches: Vec<_> = candidates.iter()
			.filter_map(|c| c.as_str())
			.filter(|c| re.is_match(c))
			.collect();
		Ok(json!({"matches": matches}))
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use serde_json::json;

	#[tokio::test]
	async fn test_valid_analogy() {
		let skill = AnalogyReasoningSkill;
		let input = json!({
			"a": "man",
			"b": "king",
			"c": "woman",
			"candidates": ["queen", "prince", "duke"]
		});
		let output = skill.execute(input).await.unwrap();
		assert_eq!(output, json!({"best_match": "queen"}));
	}

	#[tokio::test]
	async fn test_missing_candidates() {
		let skill = AnalogyReasoningSkill;
		let input = json!({
			"a": "man",
			"b": "king",
			"c": "woman"
		});
		assert!(skill.execute(input).await.is_err());
	}
}
use pandora_core::interfaces::skills::{SkillModule, SkillDescriptor, SkillOutput};
use serde_json::Value as SkillInput;
use async_trait::async_trait;
use serde_json::json;

pub struct AnalogyReasoningSkill;

#[async_trait]
impl SkillModule for AnalogyReasoningSkill {
	fn descriptor(&self) -> SkillDescriptor {
		SkillDescriptor {
			name: "analogy_reasoning".to_string(),
			description: "Suy luận tương tự: A:B :: C:?".to_string(),
			input_schema: r#"{"type":"object","properties":{"a":{"type":"string"},"b":{"type":"string"},"c":{"type":"string"},"candidates":{"type":"array","items":{"type":"string"}}},"required":["a","b","c","candidates"]}"#.to_string(),
			output_schema: r#"{"type":"object","properties":{"best_match":{"type":"string"}}}"#.to_string(),
		}
	}

	async fn execute(&self, input: SkillInput) -> SkillOutput {
		let a = input.get("a").and_then(|v| v.as_str()).ok_or("Thiếu trường 'a'")?;
		let b = input.get("b").and_then(|v| v.as_str()).ok_or("Thiếu trường 'b'")?;
		let c = input.get("c").and_then(|v| v.as_str()).ok_or("Thiếu trường 'c'")?;
		let candidates = input.get("candidates").and_then(|v| v.as_array()).ok_or("Thiếu trường 'candidates'")?;
		// Đo khoảng cách ký tự đơn giản giữa (a, b) và (c, d)
		let ab = strsim::levenshtein(a, b);
		let mut best = None;
		let mut best_score = usize::MAX;
		for cand in candidates.iter().filter_map(|v| v.as_str()) {
			let cd = strsim::levenshtein(c, cand);
			let score = (ab as isize - cd as isize).abs() as usize;
			if score < best_score {
				best = Some(cand);
				best_score = score;
			}
		}
		if let Some(ans) = best {
			Ok(json!({"best_match": ans}))
		} else {
			Err("Không tìm được đáp án phù hợp".to_string())
		}
	}
}

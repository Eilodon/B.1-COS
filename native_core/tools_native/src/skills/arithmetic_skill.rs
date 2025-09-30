#[cfg(test)]
mod tests {
	use super::*;
	use serde_json::json;

	#[tokio::test]
	async fn test_simple_addition() {
		let skill = ArithmeticSkill;
		let input = json!({"expression": "2 + 2"});
		assert_eq!(skill.execute(input).await.unwrap(), json!({"result": 4.0}));
	}

	#[tokio::test]
	async fn test_operator_precedence() {
		let skill = ArithmeticSkill;
		let input = json!({"expression": "2 + 2 * 10"});
		assert_eq!(skill.execute(input).await.unwrap(), json!({"result": 22.0}));
	}

	#[tokio::test]
	async fn test_invalid_expression() {
		let skill = ArithmeticSkill;
		let input = json!({"expression": "hai cộng hai"});
		assert!(skill.execute(input).await.is_err());
	}
}
use core_native::interfaces::skills::{SkillModule, SkillDescriptor, SkillOutput};
use serde_json::Value as SkillInput;
use async_trait::async_trait;
use serde_json::json;
use meval::eval_str;

pub struct ArithmeticSkill;

#[async_trait]
impl SkillModule for ArithmeticSkill {
	fn descriptor(&self) -> SkillDescriptor {
		SkillDescriptor {
			name: "arithmetic".to_string(),
			description: "Thực hiện phép tính số học từ biểu thức chuỗi (ví dụ: '2 + 3 * 4').".to_string(),
			input_schema: r#"{"type":"object","properties":{"expression":{"type":"string"}},"required":["expression"]}"#.to_string(),
			output_schema: r#"{"type":"object","properties":{"result":{"type":"number"}}}"#.to_string(),
		}
	}

	async fn execute(&self, input: SkillInput) -> SkillOutput {
		let expr = input.get("expression").and_then(|v| v.as_str()).ok_or("Thiếu trường 'expression'")?;
		match eval_str(expr) {
			Ok(result) => Ok(json!({"result": result})),
			Err(e) => Err(format!("Lỗi tính toán: {}", e)),
		}
	}
}

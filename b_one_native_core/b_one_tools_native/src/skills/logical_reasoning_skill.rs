#[cfg(test)]
mod tests {
	use super::*;
	use serde_json::json;

	#[tokio::test]
	async fn test_simple_and_true() {
		let skill = LogicalReasoningSkill;
		let context = json!({"is_raining": true, "time": 18});
		let ast = json!({
			"type": "AND",
			"children": [
				{"type": "VAR", "name": "is_raining"},
				{"type": "CONST", "value": true}
			]
		});
		let input = json!({"ast": ast, "context": context});
		assert_eq!(skill.execute(input).await.unwrap(), json!({"result": true}));
	}

	#[tokio::test]
	async fn test_simple_and_false() {
		let skill = LogicalReasoningSkill;
		let context = json!({"is_raining": false, "time": 18});
		let ast = json!({
			"type": "AND",
			"children": [
				{"type": "VAR", "name": "is_raining"},
				{"type": "CONST", "value": true}
			]
		});
		let input = json!({"ast": ast, "context": context});
		assert_eq!(skill.execute(input).await.unwrap(), json!({"result": false}));
	}

	#[tokio::test]
	async fn test_simple_or_true() {
		let skill = LogicalReasoningSkill;
		let context = json!({"is_raining": false, "time": 18});
		let ast = json!({
			"type": "OR",
			"children": [
				{"type": "VAR", "name": "is_raining"},
				{"type": "CONST", "value": true}
			]
		});
		let input = json!({"ast": ast, "context": context});
		assert_eq!(skill.execute(input).await.unwrap(), json!({"result": true}));
	}
}
use b_one_core_native::interfaces::skills::{SkillModule, SkillDescriptor, SkillOutput};
use serde_json::Value as SkillInput;
use async_trait::async_trait;
use serde_json::{json, Value};

pub struct LogicalReasoningSkill;

#[async_trait]
impl SkillModule for LogicalReasoningSkill {
	fn descriptor(&self) -> SkillDescriptor {
		SkillDescriptor {
			name: "logical_reasoning".to_string(),
			description: "Đánh giá các biểu thức logic dạng cây AST (AND, OR, NOT, VAR, CONST).".to_string(),
			input_schema: r#"{"type":"object","properties":{"ast":{"type":"object"},"context":{"type":"object"}},"required":["ast","context"]}"#.to_string(),
			output_schema: r#"{"type":"object","properties":{"result":{"type":"boolean"}}}"#.to_string(),
		}
	}

	async fn execute(&self, input: SkillInput) -> SkillOutput {
		let ast = input.get("ast").ok_or("Thiếu trường 'ast'")?;
		let context = input.get("context").and_then(|v| v.as_object()).ok_or("Thiếu hoặc sai kiểu 'context'")?;
		match self.evaluate_node(ast, context) {
			Ok(result) => Ok(json!({"result": result})),
			Err(e) => Err(e),
		}
	}
}

impl LogicalReasoningSkill {
	fn evaluate_node(&self, node: &Value, context: &serde_json::Map<String, Value>) -> Result<bool, String> {
	let node_type = node.get("type").and_then(|v| v.as_str()).ok_or_else(|| "Thiếu trường 'type'".to_string())?;
		match node_type {
			"CONST" => node.get("value").and_then(|v| v.as_bool()).ok_or_else(|| "Thiếu hoặc sai kiểu 'value' cho CONST".to_string()),
			"VAR" => {
				let var = node.get("name").and_then(|v| v.as_str()).ok_or_else(|| "Thiếu trường 'name' cho VAR".to_string())?;
				context.get(var).and_then(|v| v.as_bool()).ok_or_else(|| format!("Không tìm thấy biến '{}' trong context", var))
			},
			"NOT" => {
				let child = node.get("child").ok_or_else(|| "Thiếu trường 'child' cho NOT".to_string())?;
				Ok(!self.evaluate_node(child, context)?)
			},
			"AND" => {
				let children = node.get("children").and_then(|v| v.as_array()).ok_or_else(|| "Thiếu hoặc sai kiểu 'children' cho AND".to_string())?;
				for child in children {
					if !self.evaluate_node(child, context)? {
						return Ok(false);
					}
				}
				Ok(true)
			},
			"OR" => {
				let children = node.get("children").and_then(|v| v.as_array()).ok_or_else(|| "Thiếu hoặc sai kiểu 'children' cho OR".to_string())?;
				for child in children {
					if self.evaluate_node(child, context)? {
						return Ok(true);
					}
				}
				Ok(false)
			},
			_ => Err(format!("Không hỗ trợ node type: {}", node_type)),
		}
	}
}

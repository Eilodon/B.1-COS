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
use async_trait::async_trait;
use pandora_core::interfaces::skills::{SkillDescriptor, SkillModule, SkillOutput};
use pandora_error::PandoraError;
use serde_json::json;
use serde_json::Value as SkillInput;

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
        let expr = input.get("expression").and_then(|v| v.as_str()).ok_or(
            PandoraError::SkillExecution {
                skill_name: "arithmetic".into(),
                message: "Thiếu trường 'expression'".into(),
            },
        )?;
        match simple_eval(expr) {
            Ok(result) => Ok(json!({"result": result})),
            Err(e) => Err(PandoraError::SkillExecution {
                skill_name: "arithmetic".into(),
                message: format!("Lỗi tính toán: {}", e),
            }),
        }
    }
}

// Simple arithmetic evaluator without external dependencies
fn simple_eval(expr: &str) -> Result<f64, String> {
    let expr = expr.trim();
    
    // Handle simple addition
    if let Some(pos) = expr.find('+') {
        let left = expr[..pos].trim();
        let right = expr[pos + 1..].trim();
        let left_val = simple_eval(left)?;
        let right_val = simple_eval(right)?;
        return Ok(left_val + right_val);
    }
    
    // Handle simple multiplication
    if let Some(pos) = expr.find('*') {
        let left = expr[..pos].trim();
        let right = expr[pos + 1..].trim();
        let left_val = simple_eval(left)?;
        let right_val = simple_eval(right)?;
        return Ok(left_val * right_val);
    }
    
    // Handle simple subtraction
    if let Some(pos) = expr.find('-') {
        let left = expr[..pos].trim();
        let right = expr[pos + 1..].trim();
        let left_val = simple_eval(left)?;
        let right_val = simple_eval(right)?;
        return Ok(left_val - right_val);
    }
    
    // Handle simple division
    if let Some(pos) = expr.find('/') {
        let left = expr[..pos].trim();
        let right = expr[pos + 1..].trim();
        let left_val = simple_eval(left)?;
        let right_val = simple_eval(right)?;
        if right_val == 0.0 {
            return Err("Division by zero".to_string());
        }
        return Ok(left_val / right_val);
    }
    
    // Parse number
    expr.parse::<f64>().map_err(|_| format!("Invalid number: {}", expr))
}

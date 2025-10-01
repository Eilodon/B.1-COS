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
            description: "Thực hiện phép tính số học với hỗ trợ +, -, *, /, () và operator precedence".to_string(),
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
        
        match ArithmeticParser::parse(expr) {
            Ok(result) => Ok(json!({"result": result})),
            Err(e) => Err(PandoraError::SkillExecution {
                skill_name: "arithmetic".into(),
                message: format!("Lỗi tính toán: {}", e),
            }),
        }
    }
}

/// Recursive descent parser for arithmetic expressions
/// Grammar:
///   expr    -> term (("+" | "-") term)*
///   term    -> factor (("*" | "/") factor)*
///   factor  -> number | '(' expr ')'
struct ArithmeticParser {
    input: Vec<char>,
    pos: usize,
}

impl ArithmeticParser {
    fn parse(input: &str) -> Result<f64, String> {
        let mut parser = Self {
            input: input.chars().filter(|c| !c.is_whitespace()).collect(),
            pos: 0,
        };
        let result = parser.expr()?;
        
        if parser.pos < parser.input.len() {
            return Err(format!("Unexpected character at position {}", parser.pos));
        }
        
        Ok(result)
    }

    fn current(&self) -> Option<char> {
        self.input.get(self.pos).copied()
    }

    fn consume(&mut self) -> Option<char> {
        let c = self.current()?;
        self.pos += 1;
        Some(c)
    }

    fn expr(&mut self) -> Result<f64, String> {
        let mut result = self.term()?;
        
        while let Some(op) = self.current() {
            match op {
                '+' => {
                    self.consume();
                    result += self.term()?;
                }
                '-' => {
                    self.consume();
                    result -= self.term()?;
                }
                _ => break,
            }
        }
        
        Ok(result)
    }

    fn term(&mut self) -> Result<f64, String> {
        let mut result = self.factor()?;
        
        while let Some(op) = self.current() {
            match op {
                '*' => {
                    self.consume();
                    result *= self.factor()?;
                }
                '/' => {
                    self.consume();
                    let divisor = self.factor()?;
                    if divisor == 0.0 {
                        return Err("Division by zero".to_string());
                    }
                    result /= divisor;
                }
                _ => break,
            }
        }
        
        Ok(result)
    }

    fn factor(&mut self) -> Result<f64, String> {
        match self.current() {
            Some('(') => {
                self.consume(); // consume '('
                let result = self.expr()?;
                if self.consume() != Some(')') {
                    return Err("Expected closing parenthesis".to_string());
                }
                Ok(result)
            }
            Some('-') => {
                self.consume(); // consume '-'
                Ok(-self.factor()?)
            }
            Some(c) if c.is_ascii_digit() || c == '.' => self.number(),
            Some(c) => Err(format!("Unexpected character: {}", c)),
            None => Err("Unexpected end of expression".to_string()),
        }
    }

    fn number(&mut self) -> Result<f64, String> {
        let start = self.pos;
        let mut has_dot = false;
        
        while let Some(c) = self.current() {
            if c.is_ascii_digit() {
                self.consume();
            } else if c == '.' && !has_dot {
                has_dot = true;
                self.consume();
            } else {
                break;
            }
        }
        
        let num_str: String = self.input[start..self.pos].iter().copied().collect();
        if num_str.is_empty() {
            return Err("Expected number".to_string());
        }
        num_str.parse::<f64>()
            .map_err(|_| format!("Invalid number: {}", num_str))
    }
}

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
        let input = json!({"expression": "2 + 3 * 4"});
        assert_eq!(skill.execute(input).await.unwrap(), json!({"result": 14.0}));
    }

    #[tokio::test]
    async fn test_parentheses() {
        let skill = ArithmeticSkill;
        let input = json!({"expression": "(2 + 3) * 4"});
        assert_eq!(skill.execute(input).await.unwrap(), json!({"result": 20.0}));
    }

    #[tokio::test]
    async fn test_division() {
        let skill = ArithmeticSkill;
        let input = json!({"expression": "10 / 2"});
        assert_eq!(skill.execute(input).await.unwrap(), json!({"result": 5.0}));
    }

    #[tokio::test]
    async fn test_division_by_zero() {
        let skill = ArithmeticSkill;
        let input = json!({"expression": "10 / 0"});
        assert!(skill.execute(input).await.is_err());
    }

    #[tokio::test]
    async fn test_negative_numbers() {
        let skill = ArithmeticSkill;
        let input = json!({"expression": "-5 + 3"});
        assert_eq!(skill.execute(input).await.unwrap(), json!({"result": -2.0}));
    }

    #[tokio::test]
    async fn test_decimals() {
        let skill = ArithmeticSkill;
        let input = json!({"expression": "3.14 * 2"});
        assert_eq!(skill.execute(input).await.unwrap(), json!({"result": 6.28}));
    }

    #[tokio::test]
    async fn test_complex_expression() {
        let skill = ArithmeticSkill;
        let input = json!({"expression": "2 + 3 * (4 - 1) / 2"});
        assert_eq!(skill.execute(input).await.unwrap(), json!({"result": 6.5}));
    }

    #[tokio::test]
    async fn test_whitespace() {
        let skill = ArithmeticSkill;
        let input = json!({"expression": "  2   +   3  "});
        assert_eq!(skill.execute(input).await.unwrap(), json!({"result": 5.0}));
    }

    #[tokio::test]
    async fn test_invalid_expression() {
        let skill = ArithmeticSkill;
        let input = json!({"expression": "2 + + 3"});
        assert!(skill.execute(input).await.is_err());
    }
}

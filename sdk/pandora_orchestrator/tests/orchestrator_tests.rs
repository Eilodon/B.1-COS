use pandora_orchestrator::{Orchestrator, OrchestratorTrait, SkillRegistry};
use pandora_tools::skills::{
    analogy_reasoning_skill::AnalogyReasoningSkill, arithmetic_skill::ArithmeticSkill,
    logical_reasoning_skill::LogicalReasoningSkill, pattern_matching_skill::PatternMatchingSkill,
};
use serde_json::json;
use std::sync::Arc;

/// Mock skill để test error handling
struct MockSkill {
    name: String,
    should_fail: bool,
}

impl MockSkill {
    fn new(name: &str, should_fail: bool) -> Self {
        Self {
            name: name.to_string(),
            should_fail,
        }
    }
}

#[async_trait::async_trait]
impl pandora_core::interfaces::skills::SkillModule for MockSkill {
    fn descriptor(&self) -> pandora_core::interfaces::skills::SkillDescriptor {
        pandora_core::interfaces::skills::SkillDescriptor {
            name: self.name.clone(),
            description: "Mock skill for testing".to_string(),
            input_schema: "{}".to_string(),
            output_schema: "{}".to_string(),
        }
    }

    async fn execute(
        &self,
        _input: serde_json::Value,
    ) -> pandora_core::interfaces::skills::SkillOutput {
        if self.should_fail {
            Err(pandora_error::PandoraError::SkillExecution {
                skill_name: self.name.clone(),
                message: "Mock failure".to_string(),
            })
        } else {
            Ok(json!({"result": "success", "skill": self.name}))
        }
    }
}

#[tokio::test]
async fn test_skill_registry_register_and_get() {
    let mut registry = SkillRegistry::new();
    let skill = Arc::new(ArithmeticSkill);

    // Test register
    registry.register(skill.clone());

    // Test get_skill
    let retrieved_skill = registry.get_skill("arithmetic");
    assert!(retrieved_skill.is_some());

    // Test get_skill with non-existent skill
    let non_existent = registry.get_skill("non_existent");
    assert!(non_existent.is_none());
}

#[tokio::test]
async fn test_orchestrator_trait_process_request_success() {
    let mut registry = SkillRegistry::new();
    registry.register(Arc::new(ArithmeticSkill));

    let orchestrator = Orchestrator::new(Arc::new(registry));

    let input = json!({"expression": "2 + 3"});
    let result = orchestrator.process_request("arithmetic", input).await;

    assert!(result.is_ok());
    let output = result.unwrap();
    assert_eq!(output["result"], 5.0);
}

#[tokio::test]
async fn test_orchestrator_trait_process_request_skill_not_found() {
    let registry = SkillRegistry::new();
    let orchestrator = Orchestrator::new(Arc::new(registry));

    let input = json!({"expression": "2 + 3"});
    let result = orchestrator.process_request("non_existent", input).await;

    assert!(result.is_err());
    match result.unwrap_err() {
        pandora_error::PandoraError::Config(msg) => {
            assert!(msg.contains("Không tìm thấy skill"));
        }
        _ => panic!("Expected Config error"),
    }
}

#[tokio::test]
async fn test_orchestrator_trait_process_request_skill_execution_error() {
    let mut registry = SkillRegistry::new();
    registry.register(Arc::new(MockSkill::new("mock_fail", true)));

    let orchestrator = Orchestrator::new(Arc::new(registry));

    let input = json!({});
    let result = orchestrator.process_request("mock_fail", input).await;

    assert!(result.is_err());
    match result.unwrap_err() {
        pandora_error::PandoraError::SkillExecution {
            skill_name,
            message,
        } => {
            assert_eq!(skill_name, "mock_fail");
            assert_eq!(message, "Mock failure");
        }
        _ => panic!("Expected SkillExecution error"),
    }
}

#[tokio::test]
async fn test_orchestrator_trait_get_available_skills() {
    let mut registry = SkillRegistry::new();
    registry.register(Arc::new(ArithmeticSkill));
    registry.register(Arc::new(LogicalReasoningSkill));
    registry.register(Arc::new(PatternMatchingSkill));

    let orchestrator = Orchestrator::new(Arc::new(registry));
    let skills = orchestrator.get_available_skills();

    assert_eq!(skills.len(), 3);
    assert!(skills.contains(&"arithmetic".to_string()));
    assert!(skills.contains(&"logical_reasoning".to_string()));
    assert!(skills.contains(&"pattern_matching".to_string()));
}

#[tokio::test]
async fn test_orchestrator_trait_has_skill() {
    let mut registry = SkillRegistry::new();
    registry.register(Arc::new(ArithmeticSkill));

    let orchestrator = Orchestrator::new(Arc::new(registry));

    assert!(orchestrator.has_skill("arithmetic"));
    assert!(!orchestrator.has_skill("non_existent"));
}

#[tokio::test]
async fn test_orchestrator_trait_empty_registry() {
    let registry = SkillRegistry::new();
    let orchestrator = Orchestrator::new(Arc::new(registry));

    let skills = orchestrator.get_available_skills();
    assert!(skills.is_empty());

    assert!(!orchestrator.has_skill("any_skill"));
}

#[tokio::test]
async fn test_orchestrator_trait_multiple_skills_integration() {
    let mut registry = SkillRegistry::new();
    registry.register(Arc::new(ArithmeticSkill));
    registry.register(Arc::new(LogicalReasoningSkill));
    registry.register(Arc::new(PatternMatchingSkill));
    registry.register(Arc::new(AnalogyReasoningSkill));

    let orchestrator = Orchestrator::new(Arc::new(registry));

    // Test arithmetic
    let arith_result = orchestrator
        .process_request("arithmetic", json!({"expression": "10 * 5"}))
        .await;
    assert!(arith_result.is_ok());
    assert_eq!(arith_result.unwrap()["result"], 50.0);

    // Test logical reasoning
    let logic_result = orchestrator
        .process_request(
            "logical_reasoning",
            json!({
                "ast": {"type": "AND", "children": [
                    {"type": "CONST", "value": true},
                    {"type": "CONST", "value": true}
                ]},
                "context": {}
            }),
        )
        .await;
    assert!(logic_result.is_ok());
    assert_eq!(logic_result.unwrap()["result"], true);

    // Test pattern matching
    let pattern_result = orchestrator
        .process_request(
            "pattern_matching",
            json!({
                "pattern": "a*b",
                "candidates": ["ab", "aab", "b"]
            }),
        )
        .await;
    assert!(pattern_result.is_ok());
    let pattern_output = pattern_result.unwrap();
    let matches = pattern_output["matches"].as_array().unwrap();
    assert_eq!(matches.len(), 2); // "ab" and "aab"

    // Test analogy reasoning
    let analogy_result = orchestrator
        .process_request(
            "analogy_reasoning",
            json!({
                "a": "man",
                "b": "king",
                "c": "woman",
                "candidates": ["queen", "prince", "duke"]
            }),
        )
        .await;
    assert!(analogy_result.is_ok());
    let analogy_output = analogy_result.unwrap();
    assert_eq!(analogy_output["best_match"], "queen");
}

#[tokio::test]
async fn test_orchestrator_trait_error_handling_edge_cases() {
    let mut registry = SkillRegistry::new();
    registry.register(Arc::new(ArithmeticSkill));

    let orchestrator = Orchestrator::new(Arc::new(registry));

    // Test with invalid JSON input
    let invalid_input = json!({"invalid": "input"});
    let result = orchestrator
        .process_request("arithmetic", invalid_input)
        .await;
    assert!(result.is_err());

    // Test with empty skill name
    let result = orchestrator
        .process_request("", json!({"expression": "2+2"}))
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_orchestrator_trait_concurrent_requests() {
    let mut registry = SkillRegistry::new();
    registry.register(Arc::new(ArithmeticSkill));

    let orchestrator = Arc::new(Orchestrator::new(Arc::new(registry)));

    // Spawn multiple concurrent requests
    let mut handles = vec![];

    for i in 0..10 {
        let orchestrator_clone = orchestrator.clone();
        let handle = tokio::spawn(async move {
            orchestrator_clone
                .process_request(
                    "arithmetic",
                    json!({"expression": format!("{} + {}", i, i)}),
                )
                .await
        });
        handles.push(handle);
    }

    // Wait for all requests to complete
    for handle in handles {
        let result = handle.await.unwrap();
        assert!(result.is_ok());
    }
}

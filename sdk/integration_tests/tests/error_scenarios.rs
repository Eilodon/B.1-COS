use pandora_orchestrator::{Orchestrator, SkillRegistry, OrchestratorTrait};
use pandora_tools::skills::arithmetic_skill::ArithmeticSkill;
use pandora_error::PandoraError;
use std::sync::Arc;
use serde_json::json;

#[tokio::test]
async fn test_skill_not_found() {
    let registry = SkillRegistry::new();
    let orchestrator = Orchestrator::new(Arc::new(registry));

    let result = orchestrator
        .process_request("nonexistent_skill", json!({}))
        .await;

    assert!(result.is_err());
    match result.unwrap_err() {
        PandoraError::SkillNotFound { skill_name } => {
            assert_eq!(skill_name, "nonexistent_skill");
        }
        e => panic!("Expected SkillNotFound, got: {:?}", e),
    }
}

#[tokio::test]
async fn test_invalid_skill_input() {
    let mut registry = SkillRegistry::new();
    registry.register(Arc::new(ArithmeticSkill));
    let orchestrator = Orchestrator::new(Arc::new(registry));

    // Thi 0 tr 01 d 0f a 0 e1 bb 9i 'expression'
    let result = orchestrator
        .process_request("arithmetic", json!({"wrong_field": "value"}))
        .await;

    assert!(result.is_err());
    match result.unwrap_err() {
        PandoraError::InvalidSkillInput { skill_name, .. } => {
            assert_eq!(skill_name, "arithmetic");
        }
        e => panic!("Expected InvalidSkillInput, got: {:?}", e),
    }
}

#[tokio::test]
async fn test_skill_execution_error() {
    let mut registry = SkillRegistry::new();
    registry.register(Arc::new(ArithmeticSkill));
    let orchestrator = Orchestrator::new(Arc::new(registry));

    // Bi 0eu th 0bb 0d 0c 0e1 kh 0f4ng h 0b0 3p l 0ec 0a
    let result = orchestrator
        .process_request("arithmetic", json!({"expression": "invalid + + syntax"}))
        .await;

    assert!(result.is_err());
    match result.unwrap_err() {
        // ArithmeticSkill phân loại cú pháp sai là InvalidSkillInput
        PandoraError::InvalidSkillInput { skill_name, .. } => {
            assert_eq!(skill_name, "arithmetic");
        }
        e => panic!("Expected InvalidSkillInput, got: {:?}", e),
    }
}

#[tokio::test]
async fn test_division_by_zero() {
    let mut registry = SkillRegistry::new();
    registry.register(Arc::new(ArithmeticSkill));
    let orchestrator = Orchestrator::new(Arc::new(registry));

    let result = orchestrator
        .process_request("arithmetic", json!({"expression": "10 / 0"}))
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_timeout_error() {
    use pandora_orchestrator::{TimeoutPolicy, CircuitBreakerConfig};
    use tokio::time::Duration;

    // Skill ch ea m
    struct SlowSkill;

    #[async_trait::async_trait]
    impl pandora_core::interfaces::skills::SkillModule for SlowSkill {
        fn descriptor(&self) -> pandora_core::interfaces::skills::SkillDescriptor {
            pandora_core::interfaces::skills::SkillDescriptor {
                name: "slow_skill".to_string(),
                description: "A slow skill for testing".to_string(),
                input_schema: "{}".to_string(),
                output_schema: "{}".to_string(),
            }
        }

        async fn execute(&self, _input: serde_json::Value)
            -> pandora_core::interfaces::skills::SkillOutput
        {
            tokio::time::sleep(Duration::from_secs(10)).await;
            Ok(json!({"result": "done"}))
        }
    }

    let mut registry = SkillRegistry::new();
    registry.register(Arc::new(SlowSkill));

    let orchestrator = Orchestrator::with_config(
        Arc::new(registry),
        CircuitBreakerConfig::default(),
    )
    .with_timeout_policy(TimeoutPolicy { timeout_ms: 100 });

    let result = orchestrator
        .process_request("slow_skill", json!({}))
        .await;

    assert!(result.is_err());
    match result.unwrap_err() {
        PandoraError::Timeout { operation, timeout_ms } => {
            assert!(operation.contains("slow_skill"));
            assert_eq!(timeout_ms, 100);
        }
        e => panic!("Expected Timeout error, got: {:?}", e),
    }
}

#[tokio::test]
async fn test_circuit_breaker_opens() {
    use pandora_orchestrator::CircuitBreakerConfig;

    // Skill lu f4n fail
    struct FailingSkill;

    #[async_trait::async_trait]
    impl pandora_core::interfaces::skills::SkillModule for FailingSkill {
        fn descriptor(&self) -> pandora_core::interfaces::skills::SkillDescriptor {
            pandora_core::interfaces::skills::SkillDescriptor {
                name: "failing_skill".to_string(),
                description: "Always fails".to_string(),
                input_schema: "{}".to_string(),
                output_schema: "{}".to_string(),
            }
        }

        async fn execute(&self, _input: serde_json::Value)
            -> pandora_core::interfaces::skills::SkillOutput
        {
            Err(PandoraError::skill_exec("failing_skill", "Always fails"))
        }
    }

    let mut registry = SkillRegistry::new();
    registry.register(Arc::new(FailingSkill));

    let config = CircuitBreakerConfig {
        failure_threshold: 3,
        ..Default::default()
    };

    let orchestrator = Orchestrator::with_config(Arc::new(registry), config);

    // G e2y l f5i 3 l a n 11 c m df ch f1 fac m a ch e7 n
    for _ in 0..3 {
        let _ = orchestrator
            .process_request("failing_skill", json!({}))
            .await;
    }

    // G f4i ti bpt theo ph i CircuitOpen
    let result = orchestrator
        .process_request("failing_skill", json!({}))
        .await;

    assert!(result.is_err());
    match result.unwrap_err() {
        PandoraError::CircuitOpen { resource } => {
            assert_eq!(resource, "failing_skill");
        }
        e => panic!("Expected CircuitOpen, got: {:?}", e),
    }
}



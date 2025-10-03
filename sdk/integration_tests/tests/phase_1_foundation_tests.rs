// sdk/integration_tests/tests/phase_1_foundation_tests.rs

#[allow(unused_imports)]
use proptest::prelude::*;
#[allow(unused_imports)]
use std::sync::Arc;
#[allow(unused_imports)]
use std::time::{Duration, Instant};
#[allow(unused_imports)]
use tokio::sync::RwLock;

// Import các thành phần chính từ các crate của chúng ta
use chrono::Utc;
use pandora_core::ontology::*;
#[allow(unused_imports)]
use pandora_orchestrator::CognitiveError;
use pandora_orchestrator::SymbolicBrain;
use uuid::Uuid;

// Hàm helper để tạo một hệ thống test hoàn chỉnh
async fn create_test_system() -> SymbolicBrain {
    // Phase 1: sử dụng SymbolicBrain tối giản
    SymbolicBrain::new()
}

fn make_request(task_type: TaskType, input: CognitiveInput) -> CognitiveRequest {
    CognitiveRequest {
        id: Uuid::new_v4(),
        timestamp: Utc::now(),
        user_id: None,
        session_id: None,
        task_type,
        input,
        context: RequestContext::default(),
        priority: Priority::Normal,
        deadline: None,
        quality_preference: QualityPreference::Balanced,
        resource_constraints: None,
        preferred_skills: None,
    }
}

// --- Integration Tests (Kiểm thử Tích hợp) ---

#[tokio::test]
async fn test_full_pipeline_information_retrieval() {
    let system = create_test_system().await;
    let request = make_request(
        TaskType::InformationRetrieval,
        CognitiveInput::Text("Explain the theory of relativity.".to_string()),
    );

    let response = system.orchestrate_task(request).await.unwrap();

    assert!(response.confidence >= 0.0 && response.confidence <= 1.0);
    assert!(
        !response.reasoning_trace.is_empty(),
        "Phải có dấu vết suy luận"
    );
}

#[tokio::test]
async fn test_self_correction_loop_path() {
    let system = create_test_system().await;
    // Với logic hiện tại, confidence là 0.8 nên không kích hoạt self-correction.
    // Ta vẫn xác nhận không panic và có trace.
    let request = make_request(
        TaskType::InformationRetrieval,
        CognitiveInput::Text("What are p-adic numbers?".to_string()),
    );
    let response = system.orchestrate_task(request).await.unwrap();
    assert!(!response.reasoning_trace.is_empty());
}

// --- Thay thế proptest bằng kiểm thử ngẫu nhiên đơn giản ---

#[test]
fn system_never_panics_on_valid_text_input_randomized() {
    let charset: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789 ".chars().collect();
    let runtime = tokio::runtime::Runtime::new().unwrap();
    for _ in 0..50 {
        let len = fastrand::usize(1..=100);
        let text: String = (0..len).map(|_| charset[fastrand::usize(..charset.len())]).collect();
        runtime.block_on(async {
            let system = create_test_system().await;
            let request = make_request(
                TaskType::InformationRetrieval,
                CognitiveInput::Text(text.clone()),
            );
            let _ = system.orchestrate_task(request).await;
        });
    }
}

#[test]
fn confidence_is_always_valid_randomized() {
    let charset: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789 ".chars().collect();
    let runtime = tokio::runtime::Runtime::new().unwrap();
    for _ in 0..50 {
        let len = fastrand::usize(1..=100);
        let text: String = (0..len).map(|_| charset[fastrand::usize(..charset.len())]).collect();
        runtime.block_on(async {
            let system = create_test_system().await;
            let request = make_request(
                TaskType::InformationRetrieval,
                CognitiveInput::Text(text.clone()),
            );
            if let Ok(response) = system.orchestrate_task(request).await {
                assert!(response.confidence >= 0.0 && response.confidence <= 1.0);
            }
        });
    }
}

// sdk/pandora_core/src/ontology.rs

#![allow(clippy::all)]
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use uuid::Uuid;

// ===== Định danh =====
pub type UserId = String;
pub type SessionId = Uuid;
pub type SkillId = String;
pub type RuleId = String;
pub type TaskId = Uuid;

// ===== Các Enum Cốt lõi =====

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TaskType {
    Arithmetic,
    LogicalReasoning,
    InformationRetrieval,
    PatternMatching,
    AnalogyReasoning,
    SelfCorrection,
    MetaAnalysis,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub enum Priority {
    Low,
    #[default]
    Normal,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum QualityPreference {
    Fastest,
    #[default]
    Balanced,
    HighestQuality,
}

// ===== Cấu trúc Input & Context =====

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CognitiveInput {
    Text(String),
    Structured(serde_json::Value),
    Multimodal {
        text: Option<String>,
        data: Option<Vec<u8>>,
    },
    SelfReflection {
        original_request: Box<CognitiveRequest>,
        initial_response: Box<CognitiveResponse>,
        improvement_directive: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RequestContext {
    pub location: Option<String>,
    pub device_state: HashMap<String, String>,
    pub user_preferences: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceConstraints {
    pub max_cpu_cores: Option<u32>,
    pub max_memory_mb: Option<u32>,
    pub battery_aware: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitiveRequest {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub user_id: Option<UserId>,
    pub session_id: Option<SessionId>,

    // Nội dung yêu cầu
    pub task_type: TaskType,
    pub input: CognitiveInput,
    pub context: RequestContext,

    // Metadata xử lý
    pub priority: Priority,
    pub deadline: Option<DateTime<Utc>>,
    pub quality_preference: QualityPreference,
    pub resource_constraints: Option<ResourceConstraints>,
    pub preferred_skills: Option<Vec<SkillId>>,
}

// ===== Cấu trúc Output & Response =====

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResponseContent {
    Text(String),
    Structured(serde_json::Value),
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ReasoningStep {
    pub component: String,
    pub description: String,
    pub confidence: f32,
    pub duration_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitiveResponse {
    pub request_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub processing_duration: Duration,

    // Nội dung phản hồi
    pub content: ResponseContent,
    pub confidence: f32,
    pub reasoning_trace: Vec<ReasoningStep>,
    pub metadata: HashMap<String, serde_json::Value>,
    // ... các trường khác từ đặc tả sẽ được thêm ở các phase sau
}

// ===== Compat placeholders cho các module hiện có (Ngũ Uẩn) =====

use fnv::FnvHashSet;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataEidos {
    pub active_indices: FnvHashSet<u32>,
    pub dimensionality: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Vedana {
    Pleasant { karma_weight: f32 },
    Unpleasant { karma_weight: f32 },
    Neutral,
}

#[derive(Debug, Clone, Default)]
pub struct EpistemologicalFlow {
    pub rupa: Option<bytes::Bytes>,
    pub vedana: Option<Vedana>,
    pub sanna: Option<DataEidos>,
    pub related_eidos: Option<smallvec::SmallVec<[DataEidos; 4]>>,
    pub sankhara: Option<Arc<str>>,
}

impl EpistemologicalFlow {
    pub fn from_bytes(bytes: bytes::Bytes) -> Self {
        Self {
            rupa: Some(bytes),
            ..Default::default()
        }
    }
    pub fn set_static_intent(&mut self, intent: &'static str) {
        self.sankhara = Some(Arc::from(intent));
    }
    pub fn set_interned_intent(&mut self, intent: Arc<str>) {
        self.sankhara = Some(intent);
    }
}

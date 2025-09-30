use crate::interfaces::aca::{
    AcaLayer, CognitionLayer, KnowledgeLayer, ManifestationLayer, PerceptionLayer, ReflectionLayer,
    ValueLayer,
};
use crate::interfaces::fep_cell::FepCell;
use crate::ontology::CognitiveFlow;
use async_trait::async_trait;
use std::collections::HashMap;

/// Một FEP Cell có cấu trúc, điều phối một tập hợp các tầng nhận thức (ACALayers).
/// Nó không tự chứa logic nghiệp vụ, mà đóng vai trò là "bộ não" điều phối
/// luồng dữ liệu qua các "cơ quan" chức năng chuyên biệt.
pub struct OrchestratingFEPCell {
    perception: Box<dyn PerceptionLayer>,
    knowledge: Box<dyn KnowledgeLayer>,
    cognition: Box<dyn CognitionLayer>,
    value: Box<dyn ValueLayer>,
    reflection: Box<dyn ReflectionLayer>,
    manifestation: Box<dyn ManifestationLayer>,

    internal_model: HashMap<String, String>,
    prediction_error: f64,
}

impl OrchestratingFEPCell {
    pub fn new(
        perception: Box<dyn PerceptionLayer>,
        knowledge: Box<dyn KnowledgeLayer>,
        cognition: Box<dyn CognitionLayer>,
        value: Box<dyn ValueLayer>,
        reflection: Box<dyn ReflectionLayer>,
        manifestation: Box<dyn ManifestationLayer>,
    ) -> Self {
        Self {
            perception,
            knowledge,
            cognition,
            value,
            reflection,
            manifestation,
            internal_model: HashMap::new(),
            prediction_error: 0.0,
        }
    }

    /// Thực hiện một chu trình nhận thức-hành động hoàn chỉnh.
    pub async fn run_cycle(&mut self, stimulus: Vec<u8>) {
        let mut flow = CognitiveFlow { raw_stimulus: Some(stimulus), ..Default::default() };

        println!("--- Chu trình Nhận thức Bắt đầu ---");

        // 1. Cảm nhận
        self.perception.process_stimulus(&mut flow).await;
        println!("[{}] Đã xử lý kích thích.", self.perception.layer_name());

        // 2. Tri thức
        self.knowledge.represent(&mut flow).await;
        println!("[{}] Đã biểu diễn tri thức.", self.knowledge.layer_name());
        self.knowledge.retrieve(&mut flow).await;
        println!("[{}] Đã truy hồi tri thức.", self.knowledge.layer_name());

        // ... Các tầng khác sẽ được gọi ở đây trong các sprint sau ...

        println!("--- Chu trình Nhận thức Kết thúc ---");
    }
}

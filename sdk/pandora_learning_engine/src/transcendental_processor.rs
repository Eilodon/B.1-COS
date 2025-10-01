use super::LearningEngine;
use pandora_core::fep_cell::SkandhaProcessor;
use pandora_core::ontology::EpistemologicalFlow;
use pandora_core::world_model::WorldModel;
use pandora_mcg::MetaCognitiveGovernor;
use pandora_sie::SelfImprovementEngine;
use std::sync::Arc;
use tracing::info;

/// `TranscendentalProcessor` là wrapper tích hợp hoàn chỉnh tất cả các thành phần
/// để tạo ra vòng lặp siêu việt: Nhận thức -> Tự Đánh giá -> Tự Cải thiện.
pub struct TranscendentalProcessor {
    processor: SkandhaProcessor,
    learning_engine: Arc<LearningEngine>,
    mcg: MetaCognitiveGovernor,
    sie: SelfImprovementEngine,
}

impl TranscendentalProcessor {
    pub fn new(
        processor: SkandhaProcessor,
        learning_engine: Arc<LearningEngine>,
        mcg: MetaCognitiveGovernor,
        sie: SelfImprovementEngine,
    ) -> Self {
        info!("✅ TranscendentalProcessor đã được khởi tạo với vòng lặp siêu việt hoàn chỉnh.");
        Self { processor, learning_engine, mcg, sie }
    }

    /// Vận hành một chu trình Siêu Việt hoàn chỉnh.
    pub async fn run_transcendental_cycle(
        &self, 
        event: Vec<u8>,
        current_model: &dyn WorldModel,
        new_model: &dyn WorldModel,
    ) {
        info!("\n--- VÒNG LẶP SIÊU VIỆT BẮT ĐẦU ---");

        // 1. Vận hành Luồng Nhận thức Luận
        let _reborn_event = self.processor.run_epistemological_cycle(event).await;

        // 2. Tự Đánh giá (Self-Evaluation) - Tạo flow giả lập
        let flow = EpistemologicalFlow::default();
        let reward = self.learning_engine.calculate_reward(current_model, new_model, &flow);
        
        // 3. Tự Giám sát (Self-Monitoring)
        let decision = self.mcg.monitor_and_decide(&reward);
        
        // 4. Tự Cải thiện (Self-Improvement)
        match self.sie.execute(&decision).await {
            Ok(action) => {
                info!("SIE đã thực thi thành công: {}", action.description);
            }
            Err(_) => {
                info!("SIE: Không có hành động cải thiện nào được thực hiện.");
            }
        }

        // 5. Thức (có thể được bỏ qua trong vòng lặp này vì trọng tâm là tự cải thiện)
        // self.processor.vinnana.synthesize(&flow).await;
        
        info!("--- VÒNG LẶP SIÊU VIỆT KẾT THÚC ---");
    }

    /// Truy cập trực tiếp vào SkandhaProcessor gốc.
    pub fn processor(&self) -> &SkandhaProcessor {
        &self.processor
    }

    /// Truy cập trực tiếp vào LearningEngine.
    pub fn learning_engine(&self) -> &Arc<LearningEngine> {
        &self.learning_engine
    }

    /// Truy cập trực tiếp vào MCG.
    pub fn mcg(&self) -> &MetaCognitiveGovernor {
        &self.mcg
    }

    /// Truy cập trực tiếp vào SIE.
    pub fn sie(&self) -> &SelfImprovementEngine {
        &self.sie
    }
}

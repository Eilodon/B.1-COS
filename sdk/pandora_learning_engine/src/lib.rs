use pandora_core::ontology::EpistemologicalFlow;
use pandora_core::world_model::{WorldModel, DualIntrinsicReward};
use pandora_error::PandoraError;

pub mod world_models;
pub mod skandha_integration;
pub mod transcendental_processor;

pub use skandha_integration::SkandhaProcessorWithLearning;
pub use transcendental_processor::TranscendentalProcessor;

/// `LearningEngine` chịu trách nhiệm tính toán phần thưởng và điều hướng quá trình học.
pub struct LearningEngine {
    exploit_weight: f64,
    transcend_weight: f64,
}

impl LearningEngine {
    pub fn new(exploit_weight: f64, transcend_weight: f64) -> Self {
        Self { exploit_weight, transcend_weight }
    }

    /// Tính toán phần thưởng nội tại kép dựa trên sự thay đổi giữa hai mô hình.
    /// Hiện thực hóa công thức `R_intrinsic(t) = w_exploit * R_predict(t) + w_transcend * R_compress(t)`
    pub fn calculate_reward(
        &self,
        current_model: &dyn WorldModel,
        new_model: &dyn WorldModel,
        flow: &EpistemologicalFlow,
    ) -> DualIntrinsicReward {
        println!("\n--- Động Cơ Học Tập: Tính toán Phần thưởng Kép ---");

        // R_predict(t): Phần thưởng cho việc giảm sai số dự đoán.
        // Ở đây ta đơn giản hóa là lấy sai số của mô hình mới.
        let prediction_reward = -current_model.get_prediction_error(flow);
        println!("Phần thưởng Dự đoán (R_predict): {:.4}", prediction_reward);

        // R_compress(t): Phần thưởng cho việc "giác ngộ" về một quy luật đơn giản hơn.
        let compression_reward = current_model.get_mdl() - new_model.get_mdl();
        if compression_reward > 0.0 {
            println!(
                "Phần thưởng Nén (R_compress): {:.4} -> Đã tìm thấy mô hình đơn giản hơn!",
                compression_reward
            );
        }

        DualIntrinsicReward {
            prediction_reward,
            compression_reward,
        }
    }

    /// Tính toán tổng phần thưởng có trọng số.
    pub fn get_total_weighted_reward(&self, reward: &DualIntrinsicReward) -> f64 {
        let total = self.exploit_weight * reward.prediction_reward
                  + self.transcend_weight * reward.compression_reward;
        println!("=> Tổng Phần thưởng Nội tại: {:.4}", total);
        total
    }
}
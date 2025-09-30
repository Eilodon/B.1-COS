use fnv::FnvHashSet;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataEidos {
    pub active_indices: FnvHashSet<u32>,
    pub dimensionality: u32,
}

/// Cảm thọ được gán cho một sự kiện, nền tảng của nhận thức đạo đức.
#[derive(Debug, Clone, PartialEq)]
pub enum Vedana {
    /// Lạc Thọ (Pleasant): Hành động tích cực, karma_weight > 0.
    Pleasant { karma_weight: f32 },
    /// Khổ Thọ (Unpleasant): Hành động tiêu cực, karma_weight < 0.
    Unpleasant { karma_weight: f32 },
    /// Xả Thọ (Neutral): Hành động trung tính, karma_weight = 0.
    Neutral,
}

/// Dòng Chảy Nhận Thức Luận (Epistemological Flow), mang một "sự kiện" đi qua pipeline Ngũ Uẩn.
#[derive(Debug, Clone, Default)]
pub struct EpistemologicalFlow {
    /// Sắc (Form): Sự kiện nguyên thủy, hình hài của thông tin.
    pub rupa: Option<Vec<u8>>,
    
    /// Thọ (Feeling): Cảm giác đạo đức được gán cho sự kiện.
    pub vedana: Option<Vedana>,
    
    /// Tưởng (Perception): Các quy luật, mẫu hình được nhận diện.
    pub sanna: Option<DataEidos>,
    
    /// Các Chân Ảnh liên quan được truy hồi từ Tưởng.
    pub related_eidos: Option<Vec<DataEidos>>,
    
    /// Hành (Mental Formations): "Ý Chỉ" hành động được khởi phát.
    pub sankhara: Option<String>, // Placeholder for Intent
    
    // Thức (Consciousness) sẽ là kết quả cuối cùng của dòng chảy.
}

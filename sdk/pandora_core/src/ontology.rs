use fnv::FnvHashSet;
use std::sync::Arc;

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
/// Tối ưu hóa cho performance với minimal allocations.
#[derive(Debug, Clone, Default)]
pub struct EpistemologicalFlow {
    /// Sắc: Use Bytes for zero-copy slicing
    pub rupa: Option<bytes::Bytes>,
    
    /// Thọ: Inlined for cache efficiency
    pub vedana: Option<Vedana>,
    
    /// Tưởng: Compact representation
    pub sanna: Option<DataEidos>,
    
    /// Related eidos: SmallVec avoids heap for small counts
    pub related_eidos: Option<smallvec::SmallVec<[DataEidos; 4]>>,
    
    /// Hành: Use Arc<str> for cheap cloning of interned strings
    pub sankhara: Option<Arc<str>>,
    
    // Thức (Consciousness) sẽ là kết quả cuối cùng của dòng chảy.
}

impl EpistemologicalFlow {
    /// Create flow from owned bytes (zero-copy)
    pub fn from_bytes(bytes: bytes::Bytes) -> Self {
        Self {
            rupa: Some(bytes),
            ..Default::default()
        }
    }

    /// Set intent using static string (zero-copy)
    pub fn set_static_intent(&mut self, intent: &'static str) {
        self.sankhara = Some(Arc::from(intent));
    }

    /// Set intent using interned string (cheap clone)
    pub fn set_interned_intent(&mut self, intent: Arc<str>) {
        self.sankhara = Some(intent);
    }
}

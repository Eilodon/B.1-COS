use crate::interfaces::skandhas::*;
use crate::ontology::{EpistemologicalFlow, Vedana};
use async_trait::async_trait;
use tracing::{info, debug, warn};
use bytes::Bytes;
use crate::intents;

// --- 1. Sắc Uẩn ---
pub struct BasicRupaSkandha;
impl Skandha for BasicRupaSkandha {
    fn name(&self) -> &'static str { "Basic Rupa (Form)" }
}
#[async_trait]
impl RupaSkandha for BasicRupaSkandha {
    async fn process_event(&self, event: Vec<u8>) -> EpistemologicalFlow {
        info!("[{}] Tiếp nhận sự kiện nguyên thủy.", self.name());
        EpistemologicalFlow::from_bytes(Bytes::from(event))
    }
}

// --- 2. Thọ Uẩn ---
pub struct BasicVedanaSkandha;
impl Skandha for BasicVedanaSkandha {
    fn name(&self) -> &'static str { "Basic Vedana (Feeling)" }
}
impl VedanaSkandha for BasicVedanaSkandha {
    fn feel(&self, flow: &mut EpistemologicalFlow) {
        // Logic đạo đức đơn giản: Nếu event chứa từ "error", gán "Khổ Thọ".
        let feeling = if let Some(rupa) = &flow.rupa {
            if String::from_utf8_lossy(rupa.as_ref()).contains("error") {
                info!("[{}] Cảm nhận 'Khổ Thọ' từ sự kiện.", self.name());
                Vedana::Unpleasant { karma_weight: -1.0 }
            } else {
                info!("[{}] Cảm nhận 'Xả Thọ' từ sự kiện.", self.name());
                Vedana::Neutral
            }
        } else {
            Vedana::Neutral
        };
        flow.vedana = Some(feeling);
    }
}

// --- 3. Tưởng Uẩn ---
pub struct BasicSannaSkandha;
impl Skandha for BasicSannaSkandha {
    fn name(&self) -> &'static str { "Basic Sanna (Perception)" }
}
impl SannaSkandha for BasicSannaSkandha {
    fn perceive(&self, flow: &mut EpistemologicalFlow) {
        info!("[{}] Đối chiếu sự kiện, nhận diện quy luật.", self.name());
        
        // Tạo DataEidos dựa trên nội dung sự kiện
        let eidos = if let Some(rupa) = &flow.rupa {
            // Chuyển đổi sự kiện thành vector biểu diễn đơn giản
            let content = String::from_utf8_lossy(rupa.as_ref());
            let mut active_indices = std::collections::HashSet::new();
            
            // Tạo hash-based indices từ nội dung
            for (i, byte) in rupa.as_ref().iter().enumerate() {
                if *byte > 0 {
                    active_indices.insert(((i * 7) as u32 + (*byte as u32)) % 2048);
                }
            }
            
            // Thêm indices dựa trên keywords
            for keyword in ["error", "warning", "success", "info", "critical"] {
                if content.to_lowercase().contains(keyword) {
                    let hash = (keyword.len() as u32) * 13;
                    active_indices.insert(hash % 2048);
                }
            }
            
            crate::ontology::DataEidos {
                active_indices: active_indices.into_iter().collect(),
                dimensionality: 2048,
            }
        } else {
            crate::ontology::DataEidos {
                active_indices: Default::default(),
                dimensionality: 2048,
            }
        };
        
        flow.sanna = Some(eidos);
        
        // Tìm related eidos (simplified pattern matching)
        let related_eidos = self.find_related_patterns(&flow.sanna.as_ref().unwrap());
        flow.related_eidos = Some(smallvec::SmallVec::from_vec(related_eidos));
        
        info!("[{}] Đã nhận diện {} patterns liên quan.", self.name(), flow.related_eidos.as_ref().unwrap().len());
    }
}

impl BasicSannaSkandha {
    /// Tìm các patterns liên quan dựa trên DataEidos
    fn find_related_patterns(&self, eidos: &crate::ontology::DataEidos) -> Vec<crate::ontology::DataEidos> {
        let mut related = Vec::new();
        
        // Tạo một số patterns mẫu dựa trên active_indices
        for i in 0..3 {
            let mut related_eidos = eidos.clone();
            // Thêm một số indices gần kề
            for idx in eidos.active_indices.iter() {
                let new_idx = (idx + i as u32 + 1) % 2048;
                related_eidos.active_indices.insert(new_idx);
            }
            related.push(related_eidos);
        }
        
        related
    }
}

// --- 4. Hành Uẩn ---
pub struct BasicSankharaSkandha;
impl Skandha for BasicSankharaSkandha {
    fn name(&self) -> &'static str { "Basic Sankhara (Formations)" }
}
impl SankharaSkandha for BasicSankharaSkandha {
    fn form_intent(&self, flow: &mut EpistemologicalFlow) {
        // Logic đơn giản: Nếu cảm thấy "Khổ", khởi ý niệm "báo cáo lỗi".
        if let Some(Vedana::Unpleasant {..}) = flow.vedana {
            info!("[{}] Khởi phát ý chỉ: 'Báo cáo lỗi'.", self.name());
            flow.set_static_intent(intents::intents::REPORT_ERROR);
        } else {
             info!("[{}] Không có ý chỉ nào được khởi phát.", self.name());
        }
    }
}

// --- 5. Thức Uẩn ---
pub struct BasicVinnanaSkandha;
impl Skandha for BasicVinnanaSkandha {
    fn name(&self) -> &'static str { "Basic Vinnana (Consciousness)" }
}
impl VinnanaSkandha for BasicVinnanaSkandha {
    fn synthesize(&self, flow: &EpistemologicalFlow) -> Option<Vec<u8>> {
        // Logic đơn giản: Nếu có "Ý Chỉ", tổng hợp nó thành một sự kiện mới để tái sinh.
        if let Some(intent) = &flow.sankhara {
            let conscious_event = format!("Synthesized consciousness: Intent is '{}'", intent);
            info!("[{}] Tổng hợp nhận thức. Tái sinh sự kiện mới.", self.name());
            Some(conscious_event.into_bytes())
        } else {
            info!("[{}] Tổng hợp nhận thức. Vòng lặp kết thúc.", self.name());
            None
        }
    }
}

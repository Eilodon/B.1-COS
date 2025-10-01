use crate::interfaces::skandhas::*;
use crate::ontology::EpistemologicalFlow;
use crate::world_model::WorldModel;
use async_trait::async_trait;

/// `SkandhaProcessor` là FEP Cell thế hệ V3, được thiết kế để vận hành
/// Luồng Nhận Thức Luận theo pipeline Ngũ Uẩn.
pub struct SkandhaProcessor {
    rupa: Box<dyn RupaSkandha>,
    vedana: Box<dyn VedanaSkandha>,
    sanna: Box<dyn SannaSkandha>,
    sankhara: Box<dyn SankharaSkandha>,
    vinnana: Box<dyn VinnanaSkandha>,
}

impl SkandhaProcessor {
    pub fn new(
        rupa: Box<dyn RupaSkandha>,
        vedana: Box<dyn VedanaSkandha>,
        sanna: Box<dyn SannaSkandha>,
        sankhara: Box<dyn SankharaSkandha>,
        vinnana: Box<dyn VinnanaSkandha>,
    ) -> Self {
        println!("✅ SkandhaProcessor V3 đã được khởi tạo.");
        Self { rupa, vedana, sanna, sankhara, vinnana }
    }

    /// Vận hành một chu trình nhận thức Ngũ Uẩn hoàn chỉnh.
    /// Trả về một "Nhận thức" có thể được tái sinh thành sự kiện mới.
    pub async fn run_epistemological_cycle(&self, event: Vec<u8>) -> Option<Vec<u8>> {
        println!("\n--- LUỒNG NHẬN THỨC LUẬN BẮT ĐẦU ---");

        // 1. Sắc: Tiếp nhận sự kiện
        let mut flow = self.rupa.process_event(event).await;
        
        // 2. Thọ: Gán cảm giác
        self.vedana.feel(&mut flow).await;
        
        // 3. Tưởng: Nhận diện quy luật
        self.sanna.perceive(&mut flow).await;
        
        // 4. Hành: Khởi phát ý chỉ
        self.sankhara.form_intent(&mut flow).await;
        
        // 5. Thức: Tổng hợp và tái sinh
        let reborn_event = self.vinnana.synthesize(&flow).await;
        
        println!("--- LUỒNG NHẬN THỨC LUẬN KẾT THÚC ---");
        
        reborn_event
    }
}

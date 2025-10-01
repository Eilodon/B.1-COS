use crate::interfaces::skandhas::*;
use tracing::info;

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
        info!("✅ SkandhaProcessor V3 đã được khởi tạo.");
        Self {
            rupa,
            vedana,
            sanna,
            sankhara,
            vinnana,
        }
    }

    /// Vận hành một chu trình nhận thức Ngũ Uẩn hoàn chỉnh.
    /// Trả về một "Nhận thức" có thể được tái sinh thành sự kiện mới.
    pub async fn run_epistemological_cycle(&self, event: Vec<u8>) -> Option<Vec<u8>> {
        info!("\n--- LUỒNG NHẬN THỨC LUẬN BẮT ĐẦU ---");

        // 1. Sắc: Tiếp nhận sự kiện
        let mut flow = self.rupa.process_event(event).await;

        // 2. Thọ: Gán cảm giác (sync)
        self.vedana.feel(&mut flow);

        // 3. Tưởng: Nhận diện quy luật (sync)
        self.sanna.perceive(&mut flow);

        // 4. Hành: Khởi phát ý chỉ (sync)
        self.sankhara.form_intent(&mut flow);

        // 5. Thức: Tổng hợp và tái sinh (sync)
        let reborn_event = self.vinnana.synthesize(&flow);

        info!("--- LUỒNG NHẬN THỨC LUẬN KẾT THÚC ---");

        reborn_event
    }
}

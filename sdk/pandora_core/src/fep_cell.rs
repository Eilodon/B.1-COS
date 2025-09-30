use crate::interfaces::skandhas::{
    RupaSkandha, VedanaSkandha, SannaSkandha, SankharaSkandha, VinnanaSkandha,
};
use crate::ontology::EpistemologicalFlow;
use std::collections::HashMap;

/// Một FEP Cell có cấu trúc, điều phối một tập hợp các Uẩn (Skandhas).
/// Nó không tự chứa logic nghiệp vụ, mà đóng vai trò là "bộ não" điều phối
/// luồng dữ liệu qua các "cơ quan" chức năng chuyên biệt theo kiến trúc Ngũ Uẩn.
pub struct OrchestratingFEPCell {
    rupa: Box<dyn RupaSkandha>,
    vedana: Box<dyn VedanaSkandha>,
    sanna: Box<dyn SannaSkandha>,
    sankhara: Box<dyn SankharaSkandha>,
    vinnana: Box<dyn VinnanaSkandha>,

    internal_model: HashMap<String, String>,
    prediction_error: f64,
}

impl OrchestratingFEPCell {
    pub fn new(
        rupa: Box<dyn RupaSkandha>,
        vedana: Box<dyn VedanaSkandha>,
        sanna: Box<dyn SannaSkandha>,
        sankhara: Box<dyn SankharaSkandha>,
        vinnana: Box<dyn VinnanaSkandha>,
    ) -> Self {
        Self {
            rupa,
            vedana,
            sanna,
            sankhara,
            vinnana,
            internal_model: HashMap::new(),
            prediction_error: 0.0,
        }
    }

    /// Thực hiện một chu trình nhận thức-hành động hoàn chỉnh theo kiến trúc Ngũ Uẩn.
    pub async fn run_cycle(&mut self, event: Vec<u8>) {
        println!("--- Chu trình Ngũ Uẩn Bắt đầu ---");

        // 1. Sắc Uẩn: Tiếp nhận sự kiện nguyên thủy
        let mut flow = self.rupa.process_event(event).await;
        println!("[{}] Đã tiếp nhận sự kiện.", self.rupa.name());

        // 2. Thọ Uẩn: Gán cảm thọ đạo đức
        self.vedana.feel(&mut flow).await;
        println!("[{}] Đã gán cảm thọ.", self.vedana.name());

        // 3. Tưởng Uẩn: Nhận diện quy luật và mẫu hình
        self.sanna.perceive(&mut flow).await;
        println!("[{}] Đã nhận diện quy luật.", self.sanna.name());

        // 4. Hành Uẩn: Khởi phát ý chỉ hành động
        self.sankhara.form_intent(&mut flow).await;
        println!("[{}] Đã khởi phát ý chỉ.", self.sankhara.name());

        // 5. Thức Uẩn: Tổng hợp và tái sinh
        if let Some(new_event) = self.vinnana.synthesize(&flow).await {
            println!("[{}] Đã tái sinh sự kiện mới.", self.vinnana.name());
            // Có thể xử lý sự kiện mới ở đây
        } else {
            println!("[{}] Chu trình hoàn tất, không cần tái sinh.", self.vinnana.name());
        }

        println!("--- Chu trình Ngũ Uẩn Kết thúc ---");
    }
}

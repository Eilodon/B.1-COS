//! Định nghĩa FEP_Cell - Vòng lặp sống cơ bản
use async_trait::async_trait;
use crate::aca_layer::ACALayer;

#[async_trait]
pub trait FEPCell: Send + Sync {
    /// Khởi tạo cell
    async fn init(&mut self);
    /// Vòng lặp sống chính
    async fn run(&mut self);
    /// Dừng cell
    async fn stop(&mut self);
    /// Lấy tầng nhận thức hiện tại
    fn get_aca_layer(&self) -> &dyn ACALayer;
}

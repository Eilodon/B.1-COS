//! # Pandora Causal World Model (CWM)
//!
//! `pandora_cwm` là nơi chứa đựng Mô hình Thế giới Nhân quả, hay "Tàng Kinh Các Toàn Ảnh".
//! Crate này chịu trách nhiệm biểu diễn, lưu trữ và suy luận trên một cơ sở tri thức
//! được xây dựng dựa trên bản chất "Duyên Khởi".

pub mod nn;
pub mod vsa;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

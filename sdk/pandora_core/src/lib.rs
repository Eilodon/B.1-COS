//! # Pandora Core: The Soul of the SDK
//!
//! `pandora_core` là crate trung tâm, định nghĩa các khái niệm triết học và kiến trúc
//! nền tảng của Pandora Genesis SDK. Nó không chứa logic nghiệp vụ phức tạp, mà chỉ
//! cung cấp các "luật chơi" bất biến.
//!
//! Crate này bao gồm:
//!
//! - **`ontology`**: Các cấu trúc dữ liệu cốt lõi như `DataEidos` và `CognitiveFlow`.
//! - **`interfaces`**: Các giao diện (traits) trừu tượng định hình nên một thực thể nhận thức,
//!   bao gồm `FEPCell` và các tầng `ACALayer`.

pub mod interfaces;
pub mod ontology;
pub mod aca_layer;
pub mod fep_cell;

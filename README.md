# 🔱 Pandora Genesis SDK

[![Rust CI/CD](https://github.com/OWNER/REPO/actions/workflows/rust.yml/badge.svg)](https://github.com/OWNER/REPO/actions/workflows/rust.yml)
[![Coverage](https://github.com/OWNER/REPO/actions/workflows/coverage.yml/badge.svg)](https://github.com/OWNER/REPO/actions/workflows/coverage.yml)

**Pandora Genesis** là một Software Development Kit (SDK) được viết bằng Rust, được thiết kế để trở thành nền tảng cho việc xây dựng các hệ thống Trí tuệ Nhân tạo có khả năng tự cải thiện đệ quy (Recursive Self-Improvement AI).

Dự án này được xây dựng dựa trên luận đề trung tâm: **Trí tuệ thực sự không phải là kiến thức, mà là sự tinh thông trong việc học cách học (Meta-Learning Mastery).**

## 🏛️ Kiến trúc "Tâm Pháp SDK"

SDK được cấu trúc theo một triết lý ba phần rõ ràng:

1.  **Linh hồn (`pandora_core`)**: Định nghĩa các khái niệm bản thể luận, các giao diện (trait) và cấu trúc dữ liệu bất biến. Đây là bộ gen triết học của hệ thống.
2.  **Khí (`pandora_protocols`)**: Sử dụng Protocol Buffers để định nghĩa ngôn ngữ giao tiếp chung, cho phép các thành phần khác nhau (viết bằng Rust, Python, Kotlin...) tương tác một cách liền mạch.
3.  **Thân xác (`pandora_tools`)**: Cung cấp các bộ công cụ và các "skill" (kỹ năng) mẫu, là những hiện thực hóa tham khảo của các khái niệm trong `pandora_core`.

## 🚀 Bắt đầu

Để build dự án, hãy đảm bảo bạn đã cài đặt Rust toolchain:

```bash
cargo build --workspace
```

Để chạy toàn bộ các bài kiểm thử:

```bash
cargo test --workspace --all-features
```

### Chạy Coverage Report

Để tạo báo cáo coverage:

```bash
./scripts/coverage.sh
```

Hoặc chạy trực tiếp:

```bash
cd sdk
cargo llvm-cov --workspace --all-features --html --output-dir ../coverage
```

### Bật các tính năng ML (tùy chọn)

Các module liên quan đến Machine Learning (như Lượng hóa Bất định) được đặt sau feature flag `ml` để người dùng có thể tùy chọn, tránh các dependency nặng khi không cần thiết.

Để build SDK với các tính năng ML, sử dụng lệnh sau:

```bash
cargo build --workspace --features pandora_cwm/ml
```

Lệnh này sẽ kích hoạt feature `ml` trong `pandora_cwm` và các crate phụ thuộc khác như `pandora_mcg`.

### Lựa chọn skill bằng feature flags

`pandora_tools` hỗ trợ feature flags để tuỳ chọn biên dịch skill nhằm tối ưu kích thước:

- Mặc định bật tất cả: `arithmetic`, `logical_reasoning`, `information_retrieval`, `pattern_matching`, `analogy_reasoning`.
- Ví dụ chỉ bật 2 skill:

```bash
cargo build -p pandora_tools --no-default-features --features "arithmetic,pattern_matching"
```

## 🤝 Đóng góp

Chúng tôi hoan nghênh mọi sự đóng góp. Vui lòng tuân thủ các quy tắc về format và chất lượng code bằng cách chạy các lệnh sau trước khi tạo Pull Request:

```bash
cargo fmt
cargo clippy -- -D warnings
```

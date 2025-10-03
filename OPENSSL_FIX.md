# 🔧 FIX OPENSSL ISSUE - Pandora Genesis SDK

## ✅ RESOLVED - v0.2.1

**Status**: The OpenSSL dependency issue has been completely resolved in v0.2.1 by enforcing the `rustls` strategy across the board.

**Solution Applied**: 
- Configured `metrics-exporter-prometheus` to use `hyper-rustls` instead of `native-tls`
- This eliminates the need for system OpenSSL installation entirely
- Build now works on all systems without any OpenSSL dependencies

**Current Configuration**:
```toml
metrics-exporter-prometheus = { version = "0.17", default-features = false, features = ["hyper-rustls", "http-listener"] }
```

**Verification**: 
- ✅ `cargo test --workspace --all-features` passes on systems without OpenSSL
- ✅ No system dependencies required
- ✅ Cross-platform compatibility maintained

---

## 🚨 LEGACY ISSUE (RESOLVED)

Khi build với `--all-features`, gặp lỗi:
```
error: failed to run custom build command for `openssl-sys v0.9.109`
Could not find directory of OpenSSL installation
```

**Root cause**: `metrics-exporter-prometheus` → `native-tls` → `openssl-sys`

---

## ✅ GIẢI PHÁP 1: CÀI ĐẶT OPENSSL (Khuyến nghị)

### Ubuntu/Debian
```bash
sudo apt-get update
sudo apt-get install libssl-dev pkg-config
```

### Fedora/RHEL/CentOS
```bash
sudo dnf install openssl-devel pkg-config
```

### Arch Linux
```bash
sudo pacman -S openssl pkg-config
```

### macOS
```bash
brew install openssl pkg-config
```

### Verify installation
```bash
pkg-config --modversion openssl
# Should output version number like: 3.0.2
```

### Then rebuild
```bash
cd /home/ybao/B.1/B.1_COS/sdk
cargo clean
cargo build --workspace --all-features
```

---

## ✅ GIẢI PHÁP 2: SỬ DỤNG RUSTLS (Pure Rust)

### Bước 1: Update workspace dependencies

**File**: `sdk/Cargo.toml`

```toml
[workspace.dependencies]
# ... existing deps ...

# Replace native-tls với rustls
metrics-exporter-prometheus = { version = "0.14", default-features = false, features = ["rustls"] }
```

### Bước 2: Rebuild
```bash
cd /home/ybao/B.1/B.1_COS/sdk
cargo clean
cargo build --workspace --all-features
```

**Ưu điểm**:
- ✅ Pure Rust (không cần system dependencies)
- ✅ Cross-platform
- ✅ Dễ deployment

**Nhược điểm**:
- ⚠️ Có thể chậm hơn native-tls một chút
- ⚠️ Chưa support hết tất cả TLS features

---

## ✅ GIẢI PHÁP 3: DISABLE PROMETHEUS EXPORT

### Bước 1: Make it optional

**File**: `sdk/pandora_orchestrator/Cargo.toml`

```toml
[features]
default = []  # Remove prometheus_export from default
prometheus_export = ["prometheus", "metrics", "metrics-exporter-prometheus"]
```

### Bước 2: Only enable when needed
```bash
# Build without Prometheus
cargo build --workspace

# Build with Prometheus (for production)
cargo build --workspace --features pandora_orchestrator/prometheus_export
```

**Ưu điểm**:
- ✅ Không cần OpenSSL cho development
- ✅ Faster build times
- ✅ Fewer dependencies

**Nhược điểm**:
- ⚠️ Phải remember enable cho production

---

## ✅ GIẢI PHÁP 4: VENDORED OPENSSL (Portable)

### Update Cargo.toml

**File**: `sdk/Cargo.toml`

```toml
[workspace.dependencies]
# ... existing deps ...

# Add openssl with vendored feature
openssl = { version = "0.10", features = ["vendored"] }
```

### Rebuild
```bash
cd /home/ybao/B.1/B.1_COS/sdk
cargo clean
cargo build --workspace --all-features
```

**Ưu điểm**:
- ✅ Builds OpenSSL from source
- ✅ No system dependency
- ✅ Consistent across platforms

**Nhược điểm**:
- ⚠️ Longer build time (first time)
- ⚠️ Larger binary size

---

## 🎯 KHUYẾN NGHỊ

### For Development (Local)
**→ GIẢI PHÁP 1** (Install OpenSSL dev libs)
- Fastest
- Standard practice
- Best performance

### For CI/CD
**→ GIẢI PHÁP 2** (RustTLS) hoặc **GIẢI PHÁP 4** (Vendored)
- No system dependencies
- Reproducible builds
- Cross-platform

### For Production
**→ GIẢI PHÁP 1** (Native OpenSSL)
- Best performance
- Production tested
- Security updates via OS

---

## 🔍 KIỂM TRA HIỆN TRẠNG

### Check nếu OpenSSL đã được cài
```bash
# Check library
ldconfig -p | grep libssl

# Check pkg-config
pkg-config --libs openssl

# Check header files
ls -la /usr/include/openssl/
```

### Check dependency tree
```bash
cd /home/ybao/B.1/B.1_COS/sdk
cargo tree --all-features | grep -i openssl
```

Expected output:
```
│   │   │   │   ├── openssl v0.10.73
│   │   │   │   │   ├── openssl-macros v0.1.1 (proc-macro)
│   │   │   │   │   └── openssl-sys v0.9.109
```

---

## 🚀 QUICK FIX (Recommended)

### Option A: Install OpenSSL (2 phút)
```bash
# Ubuntu/Debian
sudo apt-get install -y libssl-dev pkg-config

# Verify
pkg-config --modversion openssl

# Build
cd /home/ybao/B.1/B.1_COS/sdk
cargo build --workspace --all-features
```

### Option B: Switch to RustTLS (5 phút)
```bash
cd /home/ybao/B.1/B.1_COS/sdk

# Update Cargo.toml
sed -i 's/metrics-exporter-prometheus = "0.14"/metrics-exporter-prometheus = { version = "0.14", default-features = false, features = ["rustls"] }/' Cargo.toml

# Build
cargo clean
cargo build --workspace --all-features
```

---

## ✅ VERIFICATION

Sau khi apply fix:

```bash
# Test build
cargo build --workspace --all-features

# Test clippy
cargo clippy --workspace --all-features -- -D warnings

# Test run
cargo test --workspace --all-features

# Expected output: ✅ All pass
```

---

## 📋 TROUBLESHOOTING

### Vẫn lỗi sau khi install OpenSSL?

1. **Clear cache**
   ```bash
   cargo clean
   rm -rf ~/.cargo/registry/cache
   ```

2. **Set OPENSSL_DIR manually**
   ```bash
   export OPENSSL_DIR=/usr/lib/ssl
   cargo build --workspace --all-features
   ```

3. **Check OpenSSL version**
   ```bash
   openssl version
   # Should be >= 1.1.1
   ```

### Build quá chậm?

1. **Use sccache**
   ```bash
   cargo install sccache
   export RUSTC_WRAPPER=sccache
   ```

2. **Increase parallel jobs**
   ```bash
   cargo build -j 8
   ```

3. **Disable unnecessary features**
   ```bash
   cargo build --workspace --features pandora_cwm/ml
   # Thay vì --all-features
   ```

---

## 📊 COMPARISON

| Solution | Time | Complexity | Performance | Portability |
|----------|------|------------|-------------|-------------|
| Install OpenSSL | 2 min | Low | ⭐⭐⭐⭐⭐ | ⭐⭐ |
| RustTLS | 5 min | Medium | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ |
| Disable Export | 2 min | Low | N/A | ⭐⭐⭐⭐⭐ |
| Vendored OpenSSL | 10 min | Medium | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ |

---

## 🎓 FINAL RECOMMENDATION

### IMMEDIATE ACTION (Choose ONE):

**For Ubuntu/Debian users** (FASTEST):
```bash
sudo apt-get install -y libssl-dev pkg-config
cd /home/ybao/B.1/B.1_COS/sdk
cargo build --workspace --all-features
```

**For maximum portability** (BEST for CI/CD):
```bash
cd /home/ybao/B.1/B.1_COS/sdk
# Edit Cargo.toml: add rustls feature to metrics-exporter-prometheus
cargo build --workspace --all-features
```

**For minimal dependencies** (SIMPLEST):
```bash
cd /home/ybao/B.1/B.1_COS/sdk
# Edit pandora_orchestrator/Cargo.toml: remove prometheus_export from default
cargo build --workspace
```

---

**Estimated time to fix**: 2-10 minutes depending on solution  
**Priority**: 🟡 MEDIUM (blocks --all-features only)  
**Impact**: Enables full feature compilation

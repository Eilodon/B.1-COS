plugins {
    id("com.android.library")
    kotlin("android")
}
android {
    namespace = "com.b_one.rust_core"
    compileSdk = 34
    defaultConfig {
        minSdk = 24
    }
    // Nếu cần build Rust, hãy build thủ công và copy .so vào jniLibs
}

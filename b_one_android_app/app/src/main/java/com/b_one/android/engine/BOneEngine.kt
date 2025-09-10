package com.b_one.android.engine

object BOneEngine {
    init {
        // Tên thư viện phải khớp với tên crate JNI trong Cargo.toml
        System.loadLibrary("b_one_jni")
    }
    /**
     * Hàm này tương ứng với hàm `Java_com_b_one_android_engine_BOneEngine_getCoreVersion` trong Rust.
     */
    external fun getCoreVersion(): String
}

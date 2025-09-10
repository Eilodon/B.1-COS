use jni::JNIEnv;
use jni::objects::JClass;
use jni::sys::jstring;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_com_b_one_android_engine_BOneEngine_getCoreVersion(
    env: JNIEnv,
    _class: JClass,
) -> jstring {
    let version_string = "B.ONE Native Core v0.1.0 - Ready";
    let output = env.new_string(version_string)
        .expect("Couldn't create java string!");
    output.as_raw()
}

use jni::JNIEnv;
use jni::objects::JClass;
use jni::sys::jstring;

#[no_mangle]
pub extern "system" fn Java_FFITest_greet(env: JNIEnv, class: JClass) -> jstring {
    return env.new_string("hello world!").unwrap().into_inner();
}

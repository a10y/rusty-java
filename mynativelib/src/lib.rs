use jni::JNIEnv;
use jni::objects::JClass;
use jni::sys::jstring;

#[no_mangle]
#[allow(unused_variables)]
pub extern "system" fn Java_FFITest_greet(env: JNIEnv, class: JClass) -> jstring {
    // Create a new JString object
    env.new_string("hello world!")
        // Unwrap the result
        .unwrap()
        // Dereference into lifetimeless object pointer
        .into_inner()
}

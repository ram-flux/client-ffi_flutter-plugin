#[cfg(target_os = "android")]
#[no_mangle]
pub unsafe extern "C" fn Java_com_techecho_rfapp_FFIUtil_connect_to_node(
    mut env: jni::JNIEnv,
    _class: jni::objects::JClass,
    req: jni::objects::JString,
    on_connected_callback: jni::objects::JObject,
    on_disconnected_callback: jni::objects::JObject,
    path: jni::objects::JString,
    fd: jni::sys::jint,
) -> jni::sys::jstring {
    let req: String = env.get_string(&req).unwrap().into();
    let path: String = env.get_string(&path).unwrap().into();
    let connect_req = super::serde_req(&req, &path, fd);

    let on_connected_callback = env.new_global_ref(on_connected_callback).unwrap();
    let on_disconnected_callback = env.new_global_ref(on_disconnected_callback).unwrap();
    let jvm = env.get_java_vm().unwrap();
    let callback = crate::service::android::Callback::new(
        on_connected_callback,
        on_disconnected_callback,
        jvm,
    );

    let output = connect_req.connect(callback);
    let java_string = env
        .new_string(output)
        .expect("Couldn't create java string!");
    java_string.into_raw()
}

#[cfg(target_os = "android")]
#[no_mangle]
pub unsafe extern "C" fn Java_com_techecho_rfapp_FFIUtil_disconnect(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    port: jni::sys::jint,
) -> jni::sys::jstring {
    let result = crate::service::disconnect(port as u16);
    // Create a Java string from the C string
    let java_string = env
        .new_string(result)
        .expect("Failed to create Java string");
    java_string.into_raw()
}

#[cfg(target_os = "android")]
#[no_mangle]
pub extern "C" fn Java_com_techecho_rfapp_FFIUtil_test(
    mut env: jni::JNIEnv,
    _class: jni::objects::JClass,
    str: jni::objects::JString,
) -> jni::sys::jstring {
    println!("[test] start test");
    let str: String = env.get_string(&str).unwrap().into();
    let java_string = env.new_string(str).expect("Failed to create Java string");
    java_string.into_raw()
}

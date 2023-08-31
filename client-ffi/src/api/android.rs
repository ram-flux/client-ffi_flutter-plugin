#[cfg(target_os = "android")]
#[no_mangle]
pub unsafe extern "C" fn Java_com_techecho_rfapp_FFIUtil_connectToNode(
    mut env: jni::JNIEnv,
    _class: jni::objects::JClass,
    req: jni::objects::JString,
    connect_status_callback: jni::objects::JObject,
    path: jni::objects::JString,
    fd: jni::sys::jint,
) -> jni::sys::jstring {
    let req: String = env.get_string(&req).unwrap().into();
    let path: String = env.get_string(&path).unwrap().into();
    let connect_req = super::serde_req(&req, &path, fd);
    tracing::info!("[Java_com_techecho_rfapp_FFIUtil_connectToNode] connect_req: {connect_req:#?}");

    // let on_connected_callback = env.new_global_ref(on_connected_callback).unwrap();
    let connect_status_callback = env.new_global_ref(connect_status_callback).unwrap();
    let jvm = env.get_java_vm().unwrap();
    let callback = crate::service::android::Callback::new(connect_status_callback, jvm);

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

#[cfg(target_os = "android")]
#[no_mangle]
pub extern "C" fn Java_com_techecho_rfapp_FFIUtil_syncCallback(
    mut env: jni::JNIEnv,
    _class: jni::objects::JClass,
    callback: jni::objects::JObject,
) {
    let jni_string_node = jni::strings::JNIString::from("hahaa");
    let j_string_node = env.new_string(jni_string_node).unwrap();
    let j_value_node = jni::objects::JValue::from(&j_string_node);

    let jni_string_err_msg = jni::strings::JNIString::from("eerrr");
    let j_string_err_msg = env.new_string(jni_string_err_msg).unwrap();
    let j_value_err_msg = jni::objects::JValue::from(&j_string_err_msg);

    env.call_method(
        callback,
        "onStringCallback",
        "(Ljava/lang/String;Ljava/lang/String;)V",
        // "(Ljava/lang/String;)V",
        &[j_value_node, j_value_err_msg],
    )
    .unwrap();
}

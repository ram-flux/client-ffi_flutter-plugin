


// #[cfg(target_os = "android")]
// #[no_mangle]
// pub unsafe extern fn Java_com_techecho_rfapp_FFIUtil_connect_to_node(
//     env: jni::JNIEnv,
//     _class: jni::objects::JClass,
//     req: jni::objects::JString,
//     on_connected_callback: extern fn(node_ptr: jni::objects::JString, error_message: jni::objects::JString),
//     on_disconnected_callback: extern fn(node_ptr: jni::objects::JString, error_message: jni::objects::JString),
//     path: jni::objects::JString,
//     fd: jni::sys::jint,
// ) -> jni::sys::jstring {
// //     let req = env.get_string(req).expect("invalid pattern string").as_ptr();
// //     let path = env.get_string(path).expect("invalid pattern string").as_ptr();
// //     let path = (path).expect("invalid pattern string").as_ptr();
// //     let output = connect_to_node(req, on_connected_callback, on_disconnected_callback, path, fd);
// //     let world_ptr = std::ffi::CString::from_raw(output);
// //         let output = env.new_string(world_ptr.to_str().unwrap()).expect("Couldn't create java string!");
// //     output.into_raw()
    
// }
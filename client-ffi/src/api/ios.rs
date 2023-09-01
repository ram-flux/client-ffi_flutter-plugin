#[cfg(target_os = "ios")]
use std::os::raw::c_char;

#[cfg(target_os = "ios")]
#[no_mangle]
pub extern "C" fn add(left: usize, right: usize) -> *const c_char {
    // let path = unsafe { std::ffi::CStr::from_ptr(path) }.to_str().unwrap();
    // let t = Isolate::new(port).task(loopping());
    // rt.spawn(t);
    // let res = _init_log("debug", path);

    // left + right
    std::ffi::CString::new(format!("left + right: {:?}", left + right))
        .unwrap()
        .into_raw()
}

#[cfg(target_os = "ios")]
#[no_mangle]
pub extern "C" fn test(str: *const c_char) -> *const c_char {
    println!("[test] start test");
    std::ffi::CString::new(format!(
        "str: {:?} ----- {:?}",
        str,
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
    ))
    .unwrap()
    .into_raw()
}

#[cfg(target_os = "ios")]
#[no_mangle]
pub extern "C" fn connect_to_node(
    req: *const c_char,
    on_connected_callback: extern "C" fn(node_ptr: *const c_char, error_message: *const c_char),
    on_disconnected_callback: extern "C" fn(node_ptr: *const c_char, error_message: *const c_char),
    fd: std::os::raw::c_int,
) -> *const c_char {
    let connect_req = match super::serde_req(req, fd) {
        Ok(connect_req) => connect_req,
        Err(e) => return e,
    };

    let callback =
        crate::service::ios::Callback::new(on_connected_callback, on_disconnected_callback);
    crate::ffi_result::to_c_string(&connect_req.connect(callback))
}

#[cfg(target_os = "ios")]
#[no_mangle]
pub unsafe extern "C" fn disconnect(port: u16) -> *const c_char {
    // let rt = runtime!();
    // rt.block_on(async move { crate::service::disconnect(port).await })
    crate::ffi_result::to_c_string(&crate::service::disconnect(port))
}

#[cfg(target_os = "ios")]
#[no_mangle]
pub extern "C" fn init_log(log_callback: extern "C" fn(msg: *const c_char)) -> *const c_char {
    crate::service::ios::log(log_callback);
    crate::ffi_result::to_c_string("init log")
}

#[cfg(target_os = "ios")]
#[no_mangle]
pub unsafe extern "C" fn reset_transport(port: u16, ip: String, protocol: String) -> *const c_char {
    // let rt = runtime!();
    // rt.block_on(async move { crate::service::disconnect(port).await })
    let add_transport_req = boringtun::rpc::http_server::service::AddTransportReq {
        port,
        protocol,
        endpoint: Some(ip),
    };
    crate::ffi_result::to_c_string(&crate::service::ios::reset_transport(add_transport_req))
}

use boringtun::x25519::{PublicKey, StaticSecret};

use allo_isolate::Isolate;
use rand_core::OsRng;
use std::os::raw::c_char;

// use crate::{cnode::CNode, ffi_result::FfiResult};
lazy_static::lazy_static! {
    static ref RUNTIME: std::io::Result<tokio::runtime::Runtime> = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .worker_threads(4)
        .thread_name("my-custom-name")
        .enable_time()
        .build();
}

macro_rules! runtime {
    () => {
        match RUNTIME.as_ref() {
            Ok(rt) => rt,
            Err(e) => {
                return std::ffi::CString::new(format!("runtime not exist: {e}"))
                    .unwrap()
                    .into_raw();
            }
        }
    };
}

#[no_mangle]
pub extern "C" fn add(port: i64, left: usize, right: usize, path: *const c_char) -> *const c_char {
    let rt = runtime!();
    let path = unsafe { std::ffi::CStr::from_ptr(path) }.to_str().unwrap();
    // let t = Isolate::new(port).task(loopping());
    // rt.spawn(t);
    let res = _init_log("debug", path);

    // left + right
    std::ffi::CString::new(format!("res: {res:?}"))
        .unwrap()
        .into_raw()
}

async fn loopping() {
    log::debug!("DEBUG lopppppppppp");
    println!("ahahhaah");
    // loop {}
}

#[no_mangle]
pub extern "C" fn connect_to_node(
    req: *const c_char,
    on_connected_callback: extern "C" fn(node_ptr: *const c_char, error_message: *const c_char),
    on_disconnected_callback: extern "C" fn(node_ptr: *const c_char, error_message: *const c_char),
    path: *const c_char,
    fd: std::os::raw::c_int,
) -> *const c_char {
    // let config = crate::config::Config::init("./config.toml");
    // let _ = _init_log(config.log_level.as_str());
    let path = unsafe { std::ffi::CStr::from_ptr(path) }.to_str().unwrap();

    let _ = _init_log("debug", path);
    // 将参数转换为 Rust 字符串
    println!("[connect_to_node]");
    let connect_req: crate::service::ConnectReq =
        match unsafe { std::ffi::CStr::from_ptr(req) }.to_str() {
            Err(e) => {
                let res = std::ffi::CString::new(format!("Invalid request json: {e}")).unwrap();
                return res.into_raw();
            }
            Ok(req) => serde_json::from_str(req).unwrap(),
        };
    connect_req.connect(on_connected_callback, on_disconnected_callback)
}

#[no_mangle]
pub unsafe extern "C" fn disconnect(port: u16) -> *const c_char {
    let rt = runtime!();
    rt.block_on(async move { crate::service::disconnect(port).await })
}

fn _init_log(level: &str, path: &str) {
    use tracing_subscriber::{
        fmt, prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt, EnvFilter,
        Registry,
    };
    let path = format!("{path}/node-rs");
    let file_appender = tracing_appender::rolling::hourly(path, "debug.log");
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(level));
    let formatting_layer = fmt::layer()
        .pretty()
        .with_writer(std::io::stderr)
        .with_writer(file_appender)
        .with_ansi(false);
    Registry::default()
        .with(env_filter)
        // ErrorLayer 可以让 color-eyre 获取到 span 的信息
        .with(tracing_error::ErrorLayer::default())
        // .with(fmt::layer())
        .with(formatting_layer)
        .init();
}

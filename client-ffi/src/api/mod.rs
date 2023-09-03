mod android;
mod ios;

#[cfg(target_os = "android")]
pub static LOG_INIT: once_cell::sync::OnceCell<()> = once_cell::sync::OnceCell::new();

#[cfg(target_os = "ios")]
pub fn serde_req(
    req: *const std::os::raw::c_char,
    fd: std::os::raw::c_int,
) -> Result<crate::service::ConnectReq, *mut std::os::raw::c_char> {
    let mut connect_req: crate::service::ConnectReq =
        match unsafe { std::ffi::CStr::from_ptr(req) }.to_str() {
            Err(e) => {
                let res = std::ffi::CString::new(format!("Invalid request json: {e}")).unwrap();
                return Err(res.into_raw());
            }
            Ok(req) => serde_json::from_str(req).unwrap(),
        };
    connect_req.start_req.fd = Some(fd);
    Ok(connect_req)
}

#[cfg(target_os = "android")]
pub fn serde_req(req: &str, path: &str, fd: std::os::raw::c_int) -> crate::service::ConnectReq {
    // let config = crate::config::Config::init("./config.toml");
    // let _ = _init_log(config.log_level.as_str());
    println!("[connect_to_node] get path");
    // let req = env.get_string(req);
    // let path = unsafe { std::ffi::CStr::from_ptr(path) }.to_str().unwrap();
    println!("[connect_to_node] start");
    LOG_INIT.get_or_init(|| {
        let _ = _init_log("debug", path);
    });
    // let _ = _init_log("debug", path);
    // 将参数转换为 Rust 字符串
    tracing::info!("[serde_req] init tracing log");
    let mut connect_req: crate::service::ConnectReq = match serde_json::from_str(req) {
        Ok(connect_req) => connect_req,
        Err(e) => {
            tracing::error!("[serde_req] parse error: {}", e.to_string());
            panic!();
        }
    };
    connect_req.start_req.fd = Some(fd);
    connect_req
}

fn _init_log(level: &str, path: &str) {
    use tracing_subscriber::{
        fmt, prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt, EnvFilter,
        Registry,
    };
    // let path = format!("{path}/node-rs");
    let file_appender = tracing_appender::rolling::hourly(path, "info.log");
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

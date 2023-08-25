use std::ffi::CString;

use libc::c_char;
#[derive(Debug, serde::Serialize)]
pub struct FfiResult<T: serde::Serialize> {
    /// Additional information, required to perform the operation
    pub code: u32,
    #[serde(rename = "type")]
    typ: String,
    pub message: String,
    pub data: Option<T>,
}

impl<T: serde::Serialize> FfiResult<T> {
    pub(crate) fn to_c_string(self) -> *const c_char {
        to_c_string(&self.to_string())
    }
}

impl<T> From<boringtun::rpc::http_server::response::Response<T>> for FfiResult<T>
where
    T: serde::Serialize + Sized,
{
    fn from(value: boringtun::rpc::http_server::response::Response<T>) -> Self {
        FfiResult {
            data: value.data,
            message: value.message,
            code: value.code,
            typ: "value".to_owned(),
        }
    }
}

impl<T: serde::Serialize> ToString for FfiResult<T> {
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

impl<T> From<T> for FfiResult<T>
where
    T: serde::Serialize + Sized,
{
    fn from(msg: T) -> Self {
        Self {
            code: 200,
            typ: "success".to_string(),
            message: String::new(),
            data: Some(msg),
        }
    }
}

impl<T> From<Result<T, crate::ffi_error::Error>> for FfiResult<T>
where
    T: serde::Serialize + Sized,
{
    fn from(res: Result<T, crate::ffi_error::Error>) -> Self {
        match res {
            Ok(ok) => ok.into(),
            Err(err) => {
                let (code, typ, message) = err.into();
                FfiResult {
                    code,
                    typ,
                    message,
                    data: None,
                }
            }
        }
    }
}

impl<T: serde::Serialize> From<FfiResult<T>> for *const c_char {
    fn from(value: FfiResult<T>) -> Self {
        let res = serde_json::to_string(&value).unwrap();
        let res = CString::new(res).expect("CString::new failed");
        res.as_ptr()
    }
}

// impl FfiResult<>

// 辅助函数，用于将 Rust 字符串转换为 C 字符串
pub(crate) fn to_c_string(s: &str) -> *const libc::c_char {
    CString::new(s).unwrap().into_raw()
}

// 回收错误消息的内存
pub(crate) unsafe fn _free_c_string(ptr: *const libc::c_char) {
    if !ptr.is_null() {
        let _ = CString::from_raw(ptr as *mut libc::c_char);
    }
}

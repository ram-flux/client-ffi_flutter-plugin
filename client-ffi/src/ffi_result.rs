use std::ffi::{CStr, CString};

#[repr(C)]
#[derive(PartialEq, Eq, Debug)]
/// Indicates the operation required from the caller
pub enum ResultType {
    /// No operation is required.
    FfiDone = 0,
    /// Some error occurred, no operation is required. Size indicates error code.
    FfiError = 1,
}

/// The return type of functions
#[repr(C)]
#[derive(Debug)]
pub struct FfiResult {
    /// The operation to be performed by the caller
    pub op: ResultType,
    /// Additional information, required to perform the operation
    pub data: *const libc::c_void,
    pub data_len: usize,
    pub error: *const libc::c_char,
    pub error_len: usize,
}

pub trait FfiData {
    // 定义 trait 方法
    fn as_raw_ptr(&self) -> *const libc::c_void;
}

impl FfiResult {
    pub fn new(
        data: *const libc::c_void,
        data_len: usize,
        error_message: Option<&str>,
    ) -> FfiResult {
        let (op, error, error_len) = match error_message {
            Some(message) => (ResultType::FfiError, Some(message), message.len()),
            None => (ResultType::FfiDone, None, 0),
        };

        FfiResult {
            op,
            data,
            data_len,
            error: error.map(to_c_string).unwrap_or(std::ptr::null()),
            error_len,
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct FfiError {
    pub(crate) message: *const libc::c_char,
}

impl FfiError {
    pub fn new(message: &str) -> FfiError {
        let c_string = CString::new(message).expect("CString::new failed");
        let c_string_ptr = c_string.into_raw();
        FfiError {
            message: c_string_ptr,
        }
    }

    // 辅助函数，用于创建没有错误消息的 FfiError 实例
    pub fn no_error() -> FfiError {
        FfiError {
            message: std::ptr::null(),
        }
    }

    pub fn get_message(&self) -> String {
        // 判断指针是否为空
        if self.message.is_null() {
            return String::new();
        }

        // 使用CStr将指针转换为Rust字符串，然后将其转换为String类型
        let c_str = unsafe { CStr::from_ptr(self.message) };
        let rust_str = c_str.to_str().unwrap_or("Invalid UTF-8");

        rust_str.to_string()
    }
}

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

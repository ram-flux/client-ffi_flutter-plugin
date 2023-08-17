use std::{
    ffi::{CStr, CString},
    os::raw::c_char,
};

use boringtun::{processor::node::Node, transport::TransportType};

use crate::ffi_result::FfiData;

#[no_mangle]
pub unsafe extern "C" fn free_node(node: *mut CNode) {
    // 释放CNode结构体的内存
    if !node.is_null() {
        let _ = Box::from_raw(node);
    }
}

impl FfiData for CNode {
    fn as_raw_ptr(&self) -> *const libc::c_void {
        self as *const CNode as *const libc::c_void
    }
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct CNode {
    // 结点公钥
    pub pubkey: *const c_char,
    // allowed_ips
    pub allowed_ip: *const c_char,
    // pub allowed_ips_len: usize,
    // endpoint
    pub endpoint: *const c_char,
    // 通道类型
    pub protocol: CTransportType,
}

impl From<Node> for CNode {
    fn from(node: Node) -> Self {
        // 将String转换为C字符串
        let pubkey = CString::new(node.pubkey).expect("Failed to create C string");

        let endpoint =
            CString::new(node.endpoint.unwrap_or_default()).expect("Failed to create C string");

        // 将allowed_ips转换为C字符串数组
        let allowed_ip = CString::new(node.allowed_ip).expect("Failed to create C string");

        // 创建C字符串指针数组
        // let allowed_ips_ptr: Vec<*const c_char> = allowed_ips
        //     .iter()
        //     .map(|ip| {
        //         let ip: *const c_char = ip.as_ptr();
        //         println!("[Node to CNode] ip: {ip:?}");
        //         ip
        //     })
        //     .collect();

        // 构造CNode结构体
        CNode {
            pubkey: pubkey.into_raw(),
            allowed_ip: allowed_ip.into_raw(),
            endpoint: endpoint.into_raw(),
            protocol: node.protocol.into(),
        }
    }
}

impl From<CNode> for Node {
    fn from(cnode: CNode) -> Self {
        // Convert C strings to Rust strings
        let pubkey = unsafe { CStr::from_ptr(cnode.pubkey).to_string_lossy().into_owned() };
        println!("[CNode to Node] pubkey: {}", pubkey);
        let allowed_ip = unsafe {
            CStr::from_ptr(cnode.allowed_ip)
                .to_string_lossy()
                .into_owned()
        };

        // let allowed_ips: Vec<String> = unsafe {
        //     let ips = std::slice::from_raw_parts(cnode.allowed_ips, cnode.allowed_ips_len);
        //     let mut vec = Vec::with_capacity(cnode.allowed_ips_len);
        //     println!(
        //         "[CNode to Node] cnode.allowed_ips_len: {}",
        //         cnode.allowed_ips_len
        //     );
        //     for i in 0..cnode.allowed_ips_len {
        //         if ips[i].is_null() {
        //             continue; // Skip null pointers
        //         }
        //         println!("[CNode to Node] ips[{i}]: {:?}", ips[i]);
        //         let ip_cstr = CStr::from_ptr(ips[i]);
        //         println!("[CNode to Node] ip: {:?}", ip_cstr);
        //         let ip_str = ip_cstr.to_string_lossy().into_owned();
        //         println!("[CNode to Node] ip: {}", ip_str);
        //         vec.push(ip_str);
        //         let _ = CString::from_raw(ips[i] as *mut c_char);
        //     }
        //     vec
        // };
        // println!("[CNode to Node] allowed_ips: {:?}", allowed_ips);

        let endpoint = if cnode.endpoint.is_null() {
            None
        } else {
            Some(unsafe {
                CStr::from_ptr(cnode.endpoint)
                    .to_string_lossy()
                    .into_owned()
            })
        };
        println!("[CNode to Node] endpoint: {:?}", endpoint);

        Node {
            pubkey,
            allowed_ip,
            endpoint,
            protocol: cnode.protocol.into(),
        }
    }
}

impl Drop for CNode {
    fn drop(&mut self) {
        // 释放C字符串的内存
        unsafe {
            let _ = CString::from_raw(self.pubkey as *mut c_char);
            let _ = CString::from_raw(self.endpoint as *mut c_char);
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum CTransportType {
    TCP,
    UDP,
    Websocket,
    QUIC,
    Unknown,
}

impl From<TransportType> for CTransportType {
    fn from(transport_type: TransportType) -> Self {
        match transport_type {
            TransportType::TCP => CTransportType::TCP,
            TransportType::UDP => CTransportType::UDP,
            TransportType::Websocket => CTransportType::Websocket,
            TransportType::QUIC => CTransportType::QUIC,
            TransportType::Unknown => CTransportType::Unknown,
        }
    }
}

impl From<CTransportType> for TransportType {
    fn from(transport_type: CTransportType) -> Self {
        match transport_type {
            CTransportType::TCP => TransportType::TCP,
            CTransportType::UDP => TransportType::UDP,
            CTransportType::Websocket => TransportType::Websocket,
            CTransportType::QUIC => TransportType::QUIC,
            CTransportType::Unknown => TransportType::Unknown,
        }
    }
}

#[repr(C)]
pub struct RateLimit {
    // 上传限制
    pub upload: usize,
    // 下载限制
    pub download: usize,
}

#[repr(C)]
pub struct NetworkStatus {
    // 结点延迟
    pub delay: usize,
    // 带宽
    pub bandwith: usize,
}

#[cfg(test)]
mod tests {

    #[test]
    fn cnode_node() {
        let node = boringtun::processor::node::Node {
            pubkey: "pubkey".to_string(),
            allowed_ip: "ip1".to_string(),
            endpoint: Some("endpoint".to_string()),
            protocol: boringtun::transport::TransportType::TCP,
        };

        let node_c = node.clone();
        let cnode: super::CNode = node.into();
        let node_again: boringtun::processor::node::Node = cnode.into();

        assert_eq!(node_c, node_again);
    }
}

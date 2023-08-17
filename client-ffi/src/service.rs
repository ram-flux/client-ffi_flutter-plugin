use std::{ffi::CString, os::raw::c_char};

use boringtun::{processor, rpc::http_server::ffi_callback};
use serde_derive::Deserialize;

// use crate::{cnode::CNode, ffi_result::FfiResult};

#[derive(Deserialize)]
pub struct ConnectReq {
    pub start_req: boringtun::rpc::http_server::service::StartReq,
}

// #[derive(serde::Serialize, serde::Deserialize, Debug, Default, PartialEq, Eq)]
// pub struct Response<T> {
//     pub code: u32,
//     #[serde(rename = "type")]
//     typ: String,
//     pub message: String,
//     pub data: Option<T>,
// }

// impl<T> From<T> for Response<T>
// where
//     T: serde::Serialize + Sized,
// {
//     fn from(msg: T) -> Self {
//         Self {
//             code: 200,
//             typ: "success".to_string(),
//             message: String::new(),
//             data: Some(msg),
//         }
//     }
// }

impl ConnectReq {
    pub fn connect(
        self,
        on_connected_callback: extern "C" fn(node_ptr: *const c_char, error_message: *const c_char),
        on_disconnected_callback: extern "C" fn(
            // node_ptr: *const CNode,
            node_ptr: *const c_char,
            error_message: *const c_char,
        ),
    ) -> *const c_char {
        log::info!("connect..........");
        // 获取发送者
        // tracing_subscriber::fmt()
        //     .pretty()
        //     .with_max_level(tracing::Level::DEBUG)
        //     .with_writer(std::io::stdout)
        //     .init();
        let collect_tx = processor::processor_tx_generator();
        // 回调channel
        let (callback_sender, callback_recv) =
            crossbeam_channel::unbounded::<ffi_callback::Event>();
        let (ffi_sender, ffi_receiver) = crossbeam_channel::unbounded::<
            boringtun::rpc::http_server::response_v2::Response<Option<processor::node::Node>>,
        >();
        let _ = collect_tx.send(processor::Event::ClientStart(
            self.start_req,
            ffi_sender,
            callback_sender,
        ));

        let response = match ffi_receiver.recv() {
            Ok(response) => response,
            Err(e) => {
                println!("[client_start] error: {e}");
                let res = CString::new(format!("ffi receive error: {e}")).unwrap();
                return res.into_raw();
                // return FfiResult::new(
                //     std::ptr::null(),
                //     0,
                //     Some(&format!("Failed to new protocol: {e}")),
                // );
            }
        };

        match response.code {
            200 => {
                // let c_node_ptr = match response.data.flatten() {
                //     Some(node) => {
                //         // let a :CNode = node.into();
                //         let c_node = Box::new(CNode::from(node));
                //         Box::into_raw(c_node)
                //     }
                //     None => std::ptr::null(),
                // };
                let node = serde_json::to_string(&response.data).unwrap();
                let node = CString::new(node).unwrap();
                let node = node.into_raw();
                on_connected_callback(node, std::ptr::null());

                std::thread::spawn(move || {
                    let rt = tokio::runtime::Builder::new_multi_thread()
                        .enable_all()
                        .build()
                        .unwrap();
                    rt.block_on(async {
                        handle_callback_errors(callback_recv, on_disconnected_callback)
                    })
                });

                let res = CString::new("ok").unwrap();
                res.into_raw()
                // FfiResult::new(c_node_ptr as *const libc::c_void, 0, None)
            }
            _ => {
                let err = response.message;
                let error_message = CString::new(err.clone()).expect("CString::new failed");
                let error_message_ptr = error_message.as_ptr();
                on_disconnected_callback(std::ptr::null(), error_message_ptr);
                let res = CString::new("err").unwrap();
                res.into_raw()
                // FfiResult::new(std::ptr::null(), 0, Some(err.as_str()))
            }
        }
    }
}

fn handle_callback_errors(
    callback_recv: crossbeam_channel::Receiver<ffi_callback::Event>,
    on_disconnected_callback: extern "C" fn(node_ptr: *const c_char, error_message: *const c_char),
) {
    // 在异步任务中处理错误
    match callback_recv.recv() {
        Ok(event) => match event {
            ffi_callback::Event::Disconnect(node) => {
                // let node = match node {
                //     Some(node) => {
                //         let node = CNode::from(node);
                //         let c_node = Box::new(node);
                //         Box::into_raw(c_node)
                //     }
                //     None => std::ptr::null(),
                // };
                let node = serde_json::to_string(&node).unwrap();
                let node = CString::new(node).unwrap();
                let node = node.into_raw();
                on_disconnected_callback(node, std::ptr::null());
            }
        },
        Err(e) => {
            // 处理错误信息，并将错误信息保存到共享变量
            // error.code = ERROR_UNKNOWN; // 根据实际情况设置错误码
            let error_message = format!("Error in async task: {:?}", e);
            let error_message_cstring = CString::new(error_message).expect("CString::new failed");
            let error_message_ptr = error_message_cstring.as_ptr();
            on_disconnected_callback(std::ptr::null(), error_message_ptr);

            // 必须手动释放 CString 的内存
            std::mem::forget(error_message_cstring);
        }
    }
}

pub async fn disconnect(port: u16) -> *const c_char {
    let collect_tx = processor::processor_tx_generator();
    let (ffi_sender, ffi_receiver) = crossbeam_channel::unbounded::<
        boringtun::rpc::http_server::response_v2::Response<processor::node::Node>,
    >();
    let _ = collect_tx.send(processor::Event::ClientDisconnect(port, ffi_sender));

    let response = match ffi_receiver.recv() {
        Ok(response) => response,
        Err(e) => {
            let res = CString::new(format!("ffi receive error: {e}")).unwrap();
            return res.into_raw();
            // return FfiResult::new(
            //     std::ptr::null(),
            //     0,
            //     Some(&format!("Failed to new protocol: {e}")),
            // );
        }
    };
    match response.code {
        200 => {
            // let c_node_ptr = match response.data {
            //     Some(node) => {
            //         // let a :CNode = node.into();
            //         let c_node = Box::new(CNode::from(node));
            //         Box::into_raw(c_node)
            //     }
            //     None => std::ptr::null(),
            // };
            let node = serde_json::to_string(&response.data).unwrap();
            let node = CString::new(node).unwrap();
            node.into_raw()
            // FfiResult::new(c_node_ptr as *const libc::c_void, 0, None)
        }
        _ => {
            let err = response.message;
            let res = CString::new(err).unwrap();
            res.into_raw()
            // FfiResult::new(std::ptr::null(), 0, Some(err.as_str()))
        }
    }
    // let err = "aaa";
    //         let res = CString::new(err).unwrap();
    //             res.into_raw()
}

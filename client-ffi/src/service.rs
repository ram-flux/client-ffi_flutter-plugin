use std::{ffi::CString, os::raw::c_char};

use boringtun::{processor, rpc::http_server::ffi_callback};
use serde::Deserialize;

use crate::ffi_result::FfiResult;

#[derive(Deserialize)]
pub struct ConnectReq {
    pub start_req: boringtun::rpc::http_server::service::StartReq,
}

impl ConnectReq {
    pub fn connect(
        self,
        on_connected_callback: extern "C" fn(node_ptr: *const c_char, error_message: *const c_char),
        on_disconnected_callback: extern "C" fn(
            node_ptr: *const c_char,
            error_message: *const c_char,
        ),
    ) -> *const c_char {
        println!("[connect] generate processor tx ..........");
        let collect_tx = processor::processor_tx_generator();
        // create ffi channel
        let (callback_sender, callback_recv) =
            crossbeam_channel::unbounded::<ffi_callback::Event>();
        let (ffi_sender, ffi_receiver) = crossbeam_channel::unbounded::<
            boringtun::rpc::http_server::response_v2::Response<Option<processor::node::Node>>,
        >();
        // send connect cmd
        let _ = collect_tx.send(processor::Event::ClientStart(
            self.start_req,
            ffi_sender,
            callback_sender,
        ));

        let response = match ffi_receiver.recv() {
            Ok(response) => response,
            Err(e) => {
                println!("[connect] error: {e}");
                return Into::<FfiResult<()>>::into(Err(
                    crate::ffi_error::Error::FfiChannelRecvFailed(e.to_string()),
                ))
                .to_c_string();
            }
        };

        match response.code {
            200 => {
                let node = serde_json::to_string(&response.data).unwrap_or_default();
                let cstr_node = CString::new(node).unwrap();
                let node = cstr_node.into_raw();
                on_connected_callback(node, std::ptr::null());
                println!("[connect] connect success");
                std::thread::spawn(move || {
                    let rt = tokio::runtime::Builder::new_multi_thread()
                        .enable_all()
                        .build()
                        .unwrap();
                    // let t = Isolate::new(port).task(handle_callback_errors(
                    //     callback_recv,
                    //     on_disconnected_callback,
                    // ));
                    // rt.spawn(t);
                    rt.block_on(async {
                        handle_callback_errors(callback_recv, on_disconnected_callback)
                    })
                });
            }
            _ => {
                on_disconnected_callback(
                    std::ptr::null(),
                    crate::ffi_result::to_c_string(&response.message),
                );
            }
        }
        Into::<FfiResult<Option<processor::node::Node>>>::into(response).to_c_string()
    }
}

async fn handle_callback_errors(
    callback_recv: crossbeam_channel::Receiver<ffi_callback::Event>,
    on_disconnected_callback: extern "C" fn(node_ptr: *const c_char, error_message: *const c_char),
) {
    // 在异步任务中处理错误
    match callback_recv.recv() {
        Ok(event) => match event {
            ffi_callback::Event::Disconnect(node) => {
                let node = serde_json::to_string(&node).unwrap();
                on_disconnected_callback(crate::ffi_result::to_c_string(&node), std::ptr::null());
            }
        },
        Err(e) => {
            // 处理错误信息
            on_disconnected_callback(
                std::ptr::null(),
                crate::ffi_result::to_c_string(&format!("Error in callback task: {:?}", e)),
            );

            // // 必须手动释放 CString 的内存
            // std::mem::forget(error_message_cstring);
        }
    }
}

pub fn disconnect(port: u16) -> *const c_char {
    let collect_tx = processor::processor_tx_generator();
    let (ffi_sender, ffi_receiver) = crossbeam_channel::unbounded::<
        boringtun::rpc::http_server::response_v2::Response<processor::node::Node>,
    >();
    let _ = collect_tx.send(processor::Event::ClientDisconnect(port, ffi_sender));

    let response = match ffi_receiver.recv() {
        Ok(response) => response,
        Err(e) => {
            println!("[disconnect] error: {e}");
            return Into::<FfiResult<()>>::into(Err(
                crate::ffi_error::Error::FfiChannelRecvFailed(e.to_string()),
            ))
            .to_c_string();
        }
    };
    Into::<FfiResult<processor::node::Node>>::into(response).to_c_string()
}

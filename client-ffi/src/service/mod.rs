#[cfg(target_os = "android")]
pub(crate) mod android;

#[cfg(target_os = "ios")]
pub(crate) mod ios;

#[cfg(target_os = "ios")]
use ios::Callback;

#[cfg(target_os = "android")]
use android::Callback;

use boringtun::processor;
use serde::Deserialize;

use crate::ffi_result::FfiResult;

#[derive(Deserialize, Debug)]
pub struct ConnectReq {
    pub start_req: boringtun::rpc::http_server::service::ClientStartReq,
}

#[cfg(any(target_os = "android", target_os = "ios"))]
impl ConnectReq {
    pub fn connect(
        self,
        callback: Callback, // on_connected_callback: extern "C" fn(node_ptr: *const c_char, error_message: *const c_char),
                            // on_disconnected_callback: extern "C" fn(
                            //     node_ptr: *const c_char,
                            //     error_message: *const c_char,
                            // ),
    ) -> String {
        println!("[connect] generate processor tx ..........");
        let collect_tx = processor::processor_tx_generator();
        // create ffi channel
        let (callback_sender, callback_recv) =
            crossbeam_channel::unbounded::<boringtun::rpc::http_server::ffi_callback::Event>();
        let (ffi_sender, ffi_receiver) = crossbeam_channel::unbounded::<
            boringtun::rpc::http_server::response::Response<Option<processor::node::Node>>,
        >();
        // send connect cmd
        let _ = self.start_req.client_ffi_start(
            collect_tx.clone(),
            Some(ffi_sender),
            Some(callback_sender),
        );

        let response = match ffi_receiver.recv() {
            Ok(response) => response,
            Err(e) => {
                println!("[connect] error: {e}");
                return Into::<FfiResult<()>>::into(Err(
                    crate::ffi_error::Error::FfiChannelRecvFailed(e.to_string()),
                ))
                .to_string();
            }
        };

        match response.code {
            200 => {
                let node = serde_json::to_string(&response.data).unwrap_or_default();
                callback.do_connected_callback(&node, "");
                // on_connected_callback(node, std::ptr::null());
                println!("[connect] connect success");
                std::thread::spawn(move || {
                    let rt = tokio::runtime::Builder::new_multi_thread()
                        .enable_all()
                        .build()
                        .unwrap();
                    rt.block_on(async { Self::handle_callback_errors(callback_recv, callback) })
                });
            }
            _ => {
                callback.do_disconnected_callback("", &response.message);
            }
        }

        Into::<FfiResult<Option<processor::node::Node>>>::into(response).to_string()
    }

    async fn handle_callback_errors(
        callback_recv: crossbeam_channel::Receiver<
            boringtun::rpc::http_server::ffi_callback::Event,
        >,
        callback: Callback,
        // on_disconnected_callback: extern "C" fn(node_ptr: *const c_char, error_message: *const c_char),
    ) {
        // 在异步任务中处理错误
        match callback_recv.recv() {
            Ok(event) => match event {
                boringtun::rpc::http_server::ffi_callback::Event::Disconnect(node) => {
                    let node = serde_json::to_string(&node).unwrap();
                    callback.do_disconnected_callback(&node, "");
                }
            },
            Err(e) => {
                // 处理错误信息
                callback.do_disconnected_callback("", &format!("Error in callback task: {:?}", e));

                // // 必须手动释放 CString 的内存
                // std::mem::forget(error_message_cstring);
            }
        }
    }
}

pub fn disconnect(port: u16) -> String {
    let collect_tx = processor::processor_tx_generator();
    let (ffi_sender, ffi_receiver) = crossbeam_channel::unbounded::<
        boringtun::rpc::http_server::response::Response<processor::node::Node>,
    >();
    let _ = collect_tx.send(processor::Event::ClientDisconnect(port, ffi_sender));

    let response = match ffi_receiver.recv() {
        Ok(response) => response,
        Err(e) => {
            println!("[disconnect] error: {e}");
            return Into::<FfiResult<()>>::into(Err(
                crate::ffi_error::Error::FfiChannelRecvFailed(e.to_string()),
            ))
            .to_string();
        }
    };
    Into::<FfiResult<processor::node::Node>>::into(response).to_string()
}

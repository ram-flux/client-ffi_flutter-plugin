pub struct Callback {
    on_connected_callback: extern "C" fn(node_ptr: *const c_char, error_message: *const c_char),
    on_disconnected_callback: extern "C" fn(node_ptr: *const c_char, error_message: *const c_char),
}

use libc::c_char;

impl Callback {
    pub(crate) fn new(
        on_connected_callback: extern "C" fn(node_ptr: *const c_char, error_message: *const c_char),
        on_disconnected_callback: extern "C" fn(
            node_ptr: *const c_char,
            error_message: *const c_char,
        ),
    ) -> Self {
        Callback {
            on_connected_callback,
            on_disconnected_callback,
        }
    }

    pub(crate) fn do_connected_callback(&self, node: &str, err_msg: &str) {
        let node = crate::ffi_result::to_c_string(node);
        let err_msg = crate::ffi_result::to_c_string(err_msg);

        (self.on_connected_callback)(node, err_msg)
    }
    pub(crate) fn do_disconnected_callback(&self, node: &str, err_msg: &str) {
        let node = crate::ffi_result::to_c_string(node);
        let err_msg = crate::ffi_result::to_c_string(err_msg);
        (self.on_disconnected_callback)(node, err_msg)
    }
}

pub fn log(log_callback: extern "C" fn(msg: *const c_char)) {
    let (ffi_sender, ffi_receiver) = crossbeam_channel::unbounded::<String>();
    let collect_tx = boringtun::processor::processor_tx_generator();
    let _ = collect_tx.send(boringtun::processor::Event::SetLog(ffi_sender));

    log_callback(crate::ffi_result::to_c_string("log callback init..."));

    std::thread::spawn(move || {
        log_callback(crate::ffi_result::to_c_string("handle log callback..."));
        handle_log_callback(ffi_receiver, log_callback)
    });
}

fn handle_log_callback(
    ffi_receiver: crossbeam_channel::Receiver<String>,
    log_callback: extern "C" fn(msg: *const c_char),
) {
    loop {
        match ffi_receiver.recv() {
            Ok(response) => log_callback(crate::ffi_result::to_c_string(&response)),
            Err(e) => log_callback(crate::ffi_result::to_c_string(&e.to_string())),
        };
    }
}

pub fn reset_transport(
    add_transport_req: boringtun::rpc::http_server::service::AddTransportReq,
) -> String {
    let collect_tx = boringtun::processor::processor_tx_generator();
    let (callback_sender, callback_recv) =
        crossbeam_channel::unbounded::<boringtun::rpc::http_server::ffi_callback::Event>();
    let (ffi_sender, ffi_receiver) = crossbeam_channel::unbounded();
    let res =
        add_transport_req.reset_transport(ffi_sender, Some(callback_sender), collect_tx.clone());

    tracing::info!("[reset_transport] res: {res:?}");

    let data = ffi_receiver.recv_timeout(std::time::Duration::from_secs(6));
    tracing::info!("[reset_transport] data: {data:?}");

    let response = match data {
        Ok(response) => response,
        Err(e) => {
            println!("[reset_transport] error: {e}");
            tracing::info!("[reset_transport] error: {e}");

            return Into::<crate::ffi_result::FfiResult<()>>::into(Err(
                crate::ffi_error::Error::FfiChannelRecvFailed(e.to_string()),
            ))
            .to_string();
        }
    };
    Into::<crate::ffi_result::FfiResult<Option<boringtun::processor::node::Node>>>::into(response)
        .to_string()
}

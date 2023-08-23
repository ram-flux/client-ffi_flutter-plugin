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

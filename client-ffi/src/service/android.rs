pub struct Callback {
    // on_connected_callback: jni::objects::GlobalRef,
    connect_status_callback: jni::objects::GlobalRef,
    jvm: jni::JavaVM,
}

impl Callback {
    pub(crate) fn new(
        // on_connected_callback: jni::objects::GlobalRef,
        connect_status_callback: jni::objects::GlobalRef,
        jvm: jni::JavaVM,
    ) -> Self {
        Callback {
            // on_connected_callback,
            connect_status_callback: connect_status_callback,
            jvm,
        }
    }

    pub(crate) fn do_connected_callback(&self, node: &str, err_msg: &str) {
        let mut env = self.jvm.attach_current_thread().unwrap();

        let jni_string_node = jni::strings::JNIString::from(node);
        let j_string_node = env.new_string(jni_string_node).unwrap();
        let j_value_node = jni::objects::JValue::from(&j_string_node);

        let jni_string_err_msg = jni::strings::JNIString::from(err_msg);
        let j_string_err_msg = env.new_string(jni_string_err_msg).unwrap();
        let j_value_err_msg = jni::objects::JValue::from(&j_string_err_msg);

        env.call_method(
            &self.connect_status_callback,
            "onConnectedCallback",
            "(Ljava/lang/String;Ljava/lang/String;)V",
            &[j_value_node, j_value_err_msg],
        )
        .unwrap();
    }

    pub(crate) fn do_disconnected_callback(&self, node: &str, err_msg: &str) {
        let mut env = self.jvm.attach_current_thread().unwrap();

        let jni_string_node = jni::strings::JNIString::from(node);
        let j_string_node = env.new_string(jni_string_node).unwrap();
        let j_value_node = jni::objects::JValue::from(&j_string_node);

        let jni_string_err_msg = jni::strings::JNIString::from(err_msg);
        let j_string_err_msg = env.new_string(jni_string_err_msg).unwrap();
        let j_value_err_msg = jni::objects::JValue::from(&j_string_err_msg);

        env.call_method(
            &self.connect_status_callback,
            "onDisconnectedCallback",
            "(Ljava/lang/String;Ljava/lang/String;)V",
            &[j_value_node, j_value_err_msg],
        )
        .unwrap();
    }
}

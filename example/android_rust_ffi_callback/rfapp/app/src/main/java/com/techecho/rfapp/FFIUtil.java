package com.techecho.rfapp;

public class FFIUtil {
    public static native void syncCallback(
            RustListener listener
    );

    public static native String connectToNode(String req, ConnectStatusListener connect_status_callback, String path, int fd);

    public static native String test(String str);
}


package com.techecho.rfapp;

public interface ConnectStatusListener {
    void onConnectedCallback(String node, String msg);

    void onDisconnectedCallback(String node, String msg);
}

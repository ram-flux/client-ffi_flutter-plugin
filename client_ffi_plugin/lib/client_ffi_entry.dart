import 'dart:async';
import 'dart:ffi' as ffi;

import 'package:ffi/ffi.dart';
import 'package:isolate/ports.dart';

import 'ffi.dart' as native;

class ClientFfiEntry {
  static setup() {
    native.store_dart_post_cobject(ffi.NativeApi.postCObject);
    print("Entry Setup Done");
  }

  Future<void> add(String path) async {
    var pathPointer = path.toNativeUtf8();
    final completer = Completer<bool>();
    final sendPort = singleCompletePort(completer);
    await native.add(1, 2);
  }

  String test(String str) {
    var strPointer = str.toNativeUtf8();
    final res = native.test(strPointer);
    return res.toDartString();
  }

  // Future<bool> add() {
  //   final completer = Completer<bool>();
  //   final sendPort = singleCompletePort(completer);
  //   final res = native.add(sendPort.nativePort, 1, 2);
  //   return completer.future;
  // }

  String connect(
      String req,
      ffi.Pointer<ffi.NativeFunction<OnConnectedCallback>> onConnectedCallback,
      ffi.Pointer<ffi.NativeFunction<OnDisconnectedCallback>>
          onDisconnectedCallback,
      String path) {
    // final onConnectedCallback =
    //     ffi.Pointer.fromFunction<OnConnectedCallback>(onConnectedCallbackImpl);
    // final onDisconnectedCallback =
    //     ffi.Pointer.fromFunction<OnDisconnectedCallback>(
    //         onDisconnectedCallbackImpl);
    var pathPointer = path.toNativeUtf8();
    var reqPointer = req.toNativeUtf8();
    final res = native.connect_to_node(reqPointer, onConnectedCallback,
        onDisconnectedCallback, pathPointer);
    return res.toDartString();
    // return "aaa";
  }

  String disconnect(int port) {
    final res = native.disconnect(port);
    return res.toDartString();
  }
}

typedef OnConnectedCallback = ffi.Void Function(
    ffi.Pointer<Utf8> cnode, ffi.Pointer<Utf8> message);
typedef OnDisconnectedCallback = ffi.Void Function(
    ffi.Pointer<Utf8> cnode, ffi.Pointer<Utf8> message);

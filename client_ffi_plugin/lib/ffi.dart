/// bindings for `libclient_ffi`

import 'dart:ffi';
import 'dart:io';
import 'package:ffi/ffi.dart' as ffi;

// ignore_for_file: unused_import, camel_case_types, non_constant_identifier_names
final DynamicLibrary _dl = _open();
/// Reference to the Dynamic Library, it should be only used for low-level access
final DynamicLibrary dl = _dl;
DynamicLibrary _open() {
  if (Platform.isAndroid) return DynamicLibrary.open('libclient_ffi.so');
  if (Platform.isIOS) return DynamicLibrary.executable();
  throw UnsupportedError('This platform is not supported.');
}

/// C function `Java_com_techecho_rfapp_FFIUtil_connectToNode`.
int Java_com_techecho_rfapp_FFIUtil_connectToNode(
  int env,
  int _class,
  int req,
  int connect_status_callback,
  int path,
  int fd,
) {
  return _Java_com_techecho_rfapp_FFIUtil_connectToNode(env, _class, req, connect_status_callback, path, fd);
}
final _Java_com_techecho_rfapp_FFIUtil_connectToNode_Dart _Java_com_techecho_rfapp_FFIUtil_connectToNode = _dl.lookupFunction<_Java_com_techecho_rfapp_FFIUtil_connectToNode_C, _Java_com_techecho_rfapp_FFIUtil_connectToNode_Dart>('Java_com_techecho_rfapp_FFIUtil_connectToNode');
typedef _Java_com_techecho_rfapp_FFIUtil_connectToNode_C = Int32 Function(
  Int32 env,
  Int32 _class,
  Int32 req,
  Int32 connect_status_callback,
  Int32 path,
  Int32 fd,
);
typedef _Java_com_techecho_rfapp_FFIUtil_connectToNode_Dart = int Function(
  int env,
  int _class,
  int req,
  int connect_status_callback,
  int path,
  int fd,
);

/// C function `Java_com_techecho_rfapp_FFIUtil_disconnect`.
int Java_com_techecho_rfapp_FFIUtil_disconnect(
  int env,
  int _class,
  int port,
) {
  return _Java_com_techecho_rfapp_FFIUtil_disconnect(env, _class, port);
}
final _Java_com_techecho_rfapp_FFIUtil_disconnect_Dart _Java_com_techecho_rfapp_FFIUtil_disconnect = _dl.lookupFunction<_Java_com_techecho_rfapp_FFIUtil_disconnect_C, _Java_com_techecho_rfapp_FFIUtil_disconnect_Dart>('Java_com_techecho_rfapp_FFIUtil_disconnect');
typedef _Java_com_techecho_rfapp_FFIUtil_disconnect_C = Int32 Function(
  Int32 env,
  Int32 _class,
  Int32 port,
);
typedef _Java_com_techecho_rfapp_FFIUtil_disconnect_Dart = int Function(
  int env,
  int _class,
  int port,
);

/// C function `Java_com_techecho_rfapp_FFIUtil_syncCallback`.
void Java_com_techecho_rfapp_FFIUtil_syncCallback(
  int env,
  int _class,
  int callback,
) {
  _Java_com_techecho_rfapp_FFIUtil_syncCallback(env, _class, callback);
}
final _Java_com_techecho_rfapp_FFIUtil_syncCallback_Dart _Java_com_techecho_rfapp_FFIUtil_syncCallback = _dl.lookupFunction<_Java_com_techecho_rfapp_FFIUtil_syncCallback_C, _Java_com_techecho_rfapp_FFIUtil_syncCallback_Dart>('Java_com_techecho_rfapp_FFIUtil_syncCallback');
typedef _Java_com_techecho_rfapp_FFIUtil_syncCallback_C = Void Function(
  Int32 env,
  Int32 _class,
  Int32 callback,
);
typedef _Java_com_techecho_rfapp_FFIUtil_syncCallback_Dart = void Function(
  int env,
  int _class,
  int callback,
);

/// C function `Java_com_techecho_rfapp_FFIUtil_test`.
int Java_com_techecho_rfapp_FFIUtil_test(
  int env,
  int _class,
  int str,
) {
  return _Java_com_techecho_rfapp_FFIUtil_test(env, _class, str);
}
final _Java_com_techecho_rfapp_FFIUtil_test_Dart _Java_com_techecho_rfapp_FFIUtil_test = _dl.lookupFunction<_Java_com_techecho_rfapp_FFIUtil_test_C, _Java_com_techecho_rfapp_FFIUtil_test_Dart>('Java_com_techecho_rfapp_FFIUtil_test');
typedef _Java_com_techecho_rfapp_FFIUtil_test_C = Int32 Function(
  Int32 env,
  Int32 _class,
  Int32 str,
);
typedef _Java_com_techecho_rfapp_FFIUtil_test_Dart = int Function(
  int env,
  int _class,
  int str,
);

/// C function `add`.
Pointer<ffi.Utf8> add(
  int left,
  int right,
) {
  return _add(left, right);
}
final _add_Dart _add = _dl.lookupFunction<_add_C, _add_Dart>('add');
typedef _add_C = Pointer<ffi.Utf8> Function(
  Int32 left,
  Int32 right,
);
typedef _add_Dart = Pointer<ffi.Utf8> Function(
  int left,
  int right,
);

/// C function `connect_to_node`.
Pointer<ffi.Utf8> connect_to_node(
  Pointer<ffi.Utf8> req,
  Pointer<NativeFunction<Void Function(Pointer<ffi.Utf8>, Pointer<ffi.Utf8>)>> on_connected_callback,
  Pointer<NativeFunction<Void Function(Pointer<ffi.Utf8>, Pointer<ffi.Utf8>)>> on_disconnected_callback,
  int fd,
) {
  return _connect_to_node(req, on_connected_callback, on_disconnected_callback, fd);
}
final _connect_to_node_Dart _connect_to_node = _dl.lookupFunction<_connect_to_node_C, _connect_to_node_Dart>('connect_to_node');
typedef _connect_to_node_C = Pointer<ffi.Utf8> Function(
  Pointer<ffi.Utf8> req,
  Pointer<NativeFunction<Void Function(Pointer<ffi.Utf8>, Pointer<ffi.Utf8>)>> on_connected_callback,
  Pointer<NativeFunction<Void Function(Pointer<ffi.Utf8>, Pointer<ffi.Utf8>)>> on_disconnected_callback,
  Int32 fd,
);
typedef _connect_to_node_Dart = Pointer<ffi.Utf8> Function(
  Pointer<ffi.Utf8> req,
  Pointer<NativeFunction<Void Function(Pointer<ffi.Utf8>, Pointer<ffi.Utf8>)>> on_connected_callback,
  Pointer<NativeFunction<Void Function(Pointer<ffi.Utf8>, Pointer<ffi.Utf8>)>> on_disconnected_callback,
  int fd,
);

/// C function `disconnect`.
Pointer<ffi.Utf8> disconnect(
  int port,
) {
  return _disconnect(port);
}
final _disconnect_Dart _disconnect = _dl.lookupFunction<_disconnect_C, _disconnect_Dart>('disconnect');
typedef _disconnect_C = Pointer<ffi.Utf8> Function(
  Int32 port,
);
typedef _disconnect_Dart = Pointer<ffi.Utf8> Function(
  int port,
);

/// C function `init_log`.
Pointer<ffi.Utf8> init_log(
  Pointer<NativeFunction<Void Function(Pointer<ffi.Utf8>)>> log_callback,
) {
  return _init_log(log_callback);
}
final _init_log_Dart _init_log = _dl.lookupFunction<_init_log_C, _init_log_Dart>('init_log');
typedef _init_log_C = Pointer<ffi.Utf8> Function(
  Pointer<NativeFunction<Void Function(Pointer<ffi.Utf8>)>> log_callback,
);
typedef _init_log_Dart = Pointer<ffi.Utf8> Function(
  Pointer<NativeFunction<Void Function(Pointer<ffi.Utf8>)>> log_callback,
);

/// C function `reset_transport`.
Pointer<ffi.Utf8> reset_transport(
  int port,
  Pointer<ffi.Utf8> endpoint,
  Pointer<ffi.Utf8> protocol,
) {
  return _reset_transport(port, endpoint, protocol);
}
final _reset_transport_Dart _reset_transport = _dl.lookupFunction<_reset_transport_C, _reset_transport_Dart>('reset_transport');
typedef _reset_transport_C = Pointer<ffi.Utf8> Function(
  Int32 port,
  Pointer<ffi.Utf8> endpoint,
  Pointer<ffi.Utf8> protocol,
);
typedef _reset_transport_Dart = Pointer<ffi.Utf8> Function(
  int port,
  Pointer<ffi.Utf8> endpoint,
  Pointer<ffi.Utf8> protocol,
);

/// Binding to `allo-isolate` crate
void store_dart_post_cobject(
  Pointer<NativeFunction<Int8 Function(Int64, Pointer<Dart_CObject>)>> ptr,
) {
  _store_dart_post_cobject(ptr);
}
final _store_dart_post_cobject_Dart _store_dart_post_cobject = _dl.lookupFunction<_store_dart_post_cobject_C, _store_dart_post_cobject_Dart>('store_dart_post_cobject');
typedef _store_dart_post_cobject_C = Void Function(
  Pointer<NativeFunction<Int8 Function(Int64, Pointer<Dart_CObject>)>> ptr,
);
typedef _store_dart_post_cobject_Dart = void Function(
  Pointer<NativeFunction<Int8 Function(Int64, Pointer<Dart_CObject>)>> ptr,
);

/// C function `test`.
Pointer<ffi.Utf8> test(
  Pointer<ffi.Utf8> str,
) {
  return _test(str);
}
final _test_Dart _test = _dl.lookupFunction<_test_C, _test_Dart>('test');
typedef _test_C = Pointer<ffi.Utf8> Function(
  Pointer<ffi.Utf8> str,
);
typedef _test_Dart = Pointer<ffi.Utf8> Function(
  Pointer<ffi.Utf8> str,
);

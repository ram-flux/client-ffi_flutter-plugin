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

/// C function `add`.
Pointer<ffi.Utf8> add(
  int port,
  int left,
  int right,
  Pointer<ffi.Utf8> path,
) {
  return _add(port, left, right, path);
}
final _add_Dart _add = _dl.lookupFunction<_add_C, _add_Dart>('add');
typedef _add_C = Pointer<ffi.Utf8> Function(
  Int32 port,
  Int32 left,
  Int32 right,
  Pointer<ffi.Utf8> path,
);
typedef _add_Dart = Pointer<ffi.Utf8> Function(
  int port,
  int left,
  int right,
  Pointer<ffi.Utf8> path,
);

/// C function `connect_to_node`.
Pointer<ffi.Utf8> connect_to_node(
  Pointer<ffi.Utf8> req,
  Pointer<NativeFunction<Void Function(Pointer<ffi.Utf8>, Pointer<ffi.Utf8>)>> on_connected_callback,
  Pointer<NativeFunction<Void Function(Pointer<ffi.Utf8>, Pointer<ffi.Utf8>)>> on_disconnected_callback,
  Pointer<ffi.Utf8> path,
  int fd,
) {
  return _connect_to_node(req, on_connected_callback, on_disconnected_callback, path, fd);
}
final _connect_to_node_Dart _connect_to_node = _dl.lookupFunction<_connect_to_node_C, _connect_to_node_Dart>('connect_to_node');
typedef _connect_to_node_C = Pointer<ffi.Utf8> Function(
  Pointer<ffi.Utf8> req,
  Pointer<NativeFunction<Void Function(Pointer<ffi.Utf8>, Pointer<ffi.Utf8>)>> on_connected_callback,
  Pointer<NativeFunction<Void Function(Pointer<ffi.Utf8>, Pointer<ffi.Utf8>)>> on_disconnected_callback,
  Pointer<ffi.Utf8> path,
  Int32 fd,
);
typedef _connect_to_node_Dart = Pointer<ffi.Utf8> Function(
  Pointer<ffi.Utf8> req,
  Pointer<NativeFunction<Void Function(Pointer<ffi.Utf8>, Pointer<ffi.Utf8>)>> on_connected_callback,
  Pointer<NativeFunction<Void Function(Pointer<ffi.Utf8>, Pointer<ffi.Utf8>)>> on_disconnected_callback,
  Pointer<ffi.Utf8> path,
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

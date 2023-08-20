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
  Pointer<ffi.Utf8> path,
) {
  return _connect_to_node(req, on_connected_callback, on_disconnected_callback, path);
}
final _connect_to_node_Dart _connect_to_node = _dl.lookupFunction<_connect_to_node_C, _connect_to_node_Dart>('connect_to_node');
typedef _connect_to_node_C = Pointer<ffi.Utf8> Function(
  Pointer<ffi.Utf8> req,
  Pointer<NativeFunction<Void Function(Pointer<ffi.Utf8>, Pointer<ffi.Utf8>)>> on_connected_callback,
  Pointer<NativeFunction<Void Function(Pointer<ffi.Utf8>, Pointer<ffi.Utf8>)>> on_disconnected_callback,
  Pointer<ffi.Utf8> path,
);
typedef _connect_to_node_Dart = Pointer<ffi.Utf8> Function(
  Pointer<ffi.Utf8> req,
  Pointer<NativeFunction<Void Function(Pointer<ffi.Utf8>, Pointer<ffi.Utf8>)>> on_connected_callback,
  Pointer<NativeFunction<Void Function(Pointer<ffi.Utf8>, Pointer<ffi.Utf8>)>> on_disconnected_callback,
  Pointer<ffi.Utf8> path,
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

/// C function `log`.
Pointer<ffi.Utf8> log(
  Pointer<NativeFunction<Void Function(Pointer<ffi.Utf8>)>> log_callback,
) {
  return _log(log_callback);
}
final _log_Dart _log = _dl.lookupFunction<_log_C, _log_Dart>('log');
typedef _log_C = Pointer<ffi.Utf8> Function(
  Pointer<NativeFunction<Void Function(Pointer<ffi.Utf8>)>> log_callback,
);
typedef _log_Dart = Pointer<ffi.Utf8> Function(
  Pointer<NativeFunction<Void Function(Pointer<ffi.Utf8>)>> log_callback,
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

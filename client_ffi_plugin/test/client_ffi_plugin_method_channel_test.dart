import 'package:flutter/services.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:client_ffi_plugin/client_ffi_plugin_method_channel.dart';

void main() {
  TestWidgetsFlutterBinding.ensureInitialized();

  MethodChannelClientFfiPlugin platform = MethodChannelClientFfiPlugin();
  const MethodChannel channel = MethodChannel('client_ffi_plugin');

  setUp(() {
    TestDefaultBinaryMessengerBinding.instance?.defaultBinaryMessenger.setMockMethodCallHandler(
      channel,
      (MethodCall methodCall) async {
        return '42';
      },
    );
  });

  tearDown(() {
    TestDefaultBinaryMessengerBinding.instance?.defaultBinaryMessenger.setMockMethodCallHandler(channel, null);
  });

  test('getPlatformVersion', () async {
    expect(await platform.getPlatformVersion(), '42');
  });
}

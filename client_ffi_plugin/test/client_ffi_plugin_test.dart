import 'package:flutter_test/flutter_test.dart';
import 'package:client_ffi_plugin/client_ffi_plugin.dart';
import 'package:client_ffi_plugin/client_ffi_plugin_platform_interface.dart';
import 'package:client_ffi_plugin/client_ffi_plugin_method_channel.dart';
import 'package:plugin_platform_interface/plugin_platform_interface.dart';

class MockClientFfiPluginPlatform
    with MockPlatformInterfaceMixin
    implements ClientFfiPluginPlatform {

  @override
  Future<String?> getPlatformVersion() => Future.value('42');
}

void main() {
  final ClientFfiPluginPlatform initialPlatform = ClientFfiPluginPlatform.instance;

  test('$MethodChannelClientFfiPlugin is the default instance', () {
    expect(initialPlatform, isInstanceOf<MethodChannelClientFfiPlugin>());
  });

  test('getPlatformVersion', () async {
    ClientFfiPlugin clientFfiPlugin = ClientFfiPlugin();
    MockClientFfiPluginPlatform fakePlatform = MockClientFfiPluginPlatform();
    ClientFfiPluginPlatform.instance = fakePlatform;

    expect(await clientFfiPlugin.getPlatformVersion(), '42');
  });
}

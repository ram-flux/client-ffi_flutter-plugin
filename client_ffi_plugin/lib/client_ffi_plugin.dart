
import 'client_ffi_plugin_platform_interface.dart';

class ClientFfiPlugin {
  Future<String?> getPlatformVersion() {
    return ClientFfiPluginPlatform.instance.getPlatformVersion();
  }
}

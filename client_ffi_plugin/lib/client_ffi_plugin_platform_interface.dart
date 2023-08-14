import 'package:plugin_platform_interface/plugin_platform_interface.dart';

import 'client_ffi_plugin_method_channel.dart';

abstract class ClientFfiPluginPlatform extends PlatformInterface {
  /// Constructs a ClientFfiPluginPlatform.
  ClientFfiPluginPlatform() : super(token: _token);

  static final Object _token = Object();

  static ClientFfiPluginPlatform _instance = MethodChannelClientFfiPlugin();

  /// The default instance of [ClientFfiPluginPlatform] to use.
  ///
  /// Defaults to [MethodChannelClientFfiPlugin].
  static ClientFfiPluginPlatform get instance => _instance;

  /// Platform-specific implementations should set this with their own
  /// platform-specific class that extends [ClientFfiPluginPlatform] when
  /// they register themselves.
  static set instance(ClientFfiPluginPlatform instance) {
    PlatformInterface.verifyToken(instance, _token);
    _instance = instance;
  }

  Future<String?> getPlatformVersion() {
    throw UnimplementedError('platformVersion() has not been implemented.');
  }
}

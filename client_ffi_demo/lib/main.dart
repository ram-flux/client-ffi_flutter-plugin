import 'dart:io';

import 'package:client_ffi_plugin/client_ffi_entry.dart';
// import 'package:dart_rust_logger_plugin/dart_rust_logger_entry.dart';
import 'package:ffi/ffi.dart';
import 'package:flutter/material.dart';
import 'dart:ffi' as ffi;
import 'package:path_provider/path_provider.dart';

// import 'package:log_test_plugin/log_test_entry.dart';

void main() {
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  // This widget is the root of your application.
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Flutter Demo',
      theme: ThemeData(
        // This is the theme of your application.
        //
        // TRY THIS: Try running your application with "flutter run". You'll see
        // the application has a blue toolbar. Then, without quitting the app,
        // try changing the seedColor in the colorScheme below to Colors.green
        // and then invoke "hot reload" (save your changes or press the "hot
        // reload" button in a Flutter-supported IDE, or press "r" if you used
        // the command line to start the app).
        //
        // Notice that the counter didn't reset back to zero; the application
        // state is not lost during the reload. To reset the state, use hot
        // restart instead.
        //
        // This works for code too, not just values: Most code changes can be
        // tested with just a hot reload.
        colorScheme: ColorScheme.fromSeed(seedColor: Colors.deepPurple),
        useMaterial3: true,
      ),
      home: const MyHomePage(title: 'Flutter Demo Home Page'),
    );
  }
}

class MyHomePage extends StatefulWidget {
  const MyHomePage({super.key, required this.title});

  // This widget is the home page of your application. It is stateful, meaning
  // that it has a State object (defined below) that contains fields that affect
  // how it looks.

  // This class is the configuration for the state. It holds the values (in this
  // case the title) provided by the parent (in this case the App widget) and
  // used by the build method of the State. Fields in a Widget subclass are
  // always marked "final".

  final String title;

  @override
  State<MyHomePage> createState() => _MyHomePageState();
}

class Callbacks {
  static void onConnectedCallbackImpl(
      ffi.Pointer<Utf8> node, ffi.Pointer<Utf8> msg) {
    final nodeStr = node.toDartString();
    final msgStr = msg.toDartString();
    print('Connected Callback: Request: $nodeStr, Response: $msgStr');
  }

  static void onDisconnectedCallbackImpl(
      ffi.Pointer<Utf8> node, ffi.Pointer<Utf8> msg) {
    final nodeStr = node.toDartString();
    final msgStr = msg.toDartString();
    print('Disconnected Callback: Request: $nodeStr, Response: $msgStr');
  }
}

class _MyHomePageState extends State<MyHomePage> {
  int _counter = 0;
  static ClientFfiEntry clientFfiEntry = ClientFfiEntry();
  // static DartRustLoggerEntry dartRustLoggerEntry = DartRustLoggerEntry();
  // static LogTestEntry logTestEntry = LogTestEntry();
  void _incrementCounter() {
    setState(() {
      // This call to setState tells the Flutter framework that something has
      // changed in this State, which causes it to rerun the build method below
      // so that the display can reflect the updated values. If we changed
      // _counter without calling setState(), then the build method would not be
      // called again, and so nothing would appear to happen.

      _counter++;
    });
  }

  @override
  void initState() {
    super.initState();
    ClientFfiEntry.setup();

    // DartRustLoggerEntry.setup();
  }

  Future<String> _getAppDocDirectory() async {
    Directory directory = await getApplicationDocumentsDirectory();
    return directory.path;
  }

  connect() async {
    String req = '''
                  {
                      "start_req":{
                          "type": "client",
                          "set_key_req": {
                              "prikey": "145c46d72ea427ba2875fabdfc8b97bb403caccb1313141079212fdc8844d9e3"
                          },
                          "assign_interface_req": {
                              "name": "utun",
                              "num": 100,
                              "native_external_ip": "220.200.5.244",
                              "ipv4": "10.77.0.2",
                              "ipv6": "fd86:ea04:1111::"
                          },
                          "add_transport_req": {
                              "port": 5173,
                              "protocol": "websocket",
                              "endpoint": "52.221.222.252:5173"
                          },
                          "add_network_segment_req": {
                              "segments": [
                                  "142.250.204.100/32"
                              ]
                          },
                          "add_node_req": {
                              "pub_key": "ee86c9a522e28f5299ad2f560db0174a2d4b0c67aedb90ed7defbc7afbf05e23",
                              "endpoint": "52.221.222.252:5173",
                              "allowed_ips": [
                                  "142.250.204.100/32"
                              ]
                          }
                      }
                  }
                  ''';
    final onConnected = ffi.Pointer.fromFunction<OnConnectedCallback>(
        Callbacks.onConnectedCallbackImpl);

    final onDisconnected = ffi.Pointer.fromFunction<OnDisconnectedCallback>(
        Callbacks.onDisconnectedCallbackImpl);
    return clientFfiEntry.connect(
        req, onConnected, onDisconnected, await _getAppDocDirectory());
  }

  disconnect() {
    var res = clientFfiEntry.disconnect(5173);
    print("result: ${res}");
  }

  // static FutureOr<bool> add_future(bool i) async {
  //   var res = await clientFfiEntry.add();
  //   print("sqliteServer start");
  //   return res;
  // }

  // Future<bool> add() async {
  //   var res = await ISOManager.loadBalanceFuture<bool, bool>(add_future, true);
  //   clientFfiEntry.add();
  //   print("result: ${res}");
  //   return res;
  // }

  @override
  Widget build(BuildContext context) {
    // This method is rerun every time setState is called, for instance as done
    // by the _incrementCounter method above.
    //
    // The Flutter framework has been optimized to make rerunning build methods
    // fast, so that you can just rebuild anything that needs updating rather
    // than having to individually change instances of widgets.
    return Scaffold(
      appBar: AppBar(
        // TRY THIS: Try changing the color here to a specific color (to
        // Colors.amber, perhaps?) and trigger a hot reload to see the AppBar
        // change color while the other colors stay the same.
        backgroundColor: Theme.of(context).colorScheme.inversePrimary,
        // Here we take the value from the MyHomePage object that was created by
        // the App.build method, and use it to set our appbar title.
        title: Text(widget.title),
      ),
      body: Container(
        color: Colors.green,
        child: Stack(children: [
          GridView.count(
            crossAxisCount: 5,
            children: [
              ElevatedButton(
                  onPressed: () {
                    final res = connect();
                    print("connect: $res");
                  },
                  child: const Text("连接")),
              ElevatedButton(
                  onPressed: () async {
                    final res = disconnect();
                    print("disconnect: $res");
                  },
                  child: const Text("断开连接")),
              ElevatedButton(
                  onPressed: () async {
                    final res = clientFfiEntry.add(await _getAppDocDirectory());
                    print("init: $res");
                  },
                  child: const Text("add")),
              ElevatedButton(
                  onPressed: () async {
                    final res = await _getAppDocDirectory();
                    print("init: $res");
                  },
                  child: const Text("_getAppDocDirectory")),
              // ElevatedButton(
              //     onPressed: () {
              //       final res = logTestEntry.callFfiPublishMessage("哈哈哈");
              //       print("init: $res");
              //     },
              //     child: const Text("测试别的插件日志")),
            ],
          ),
        ]),
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: _incrementCounter,
        tooltip: 'Increment',
        child: const Icon(Icons.add),
      ), // This trailing comma makes auto-formatting nicer for build methods.
    );
  }
}

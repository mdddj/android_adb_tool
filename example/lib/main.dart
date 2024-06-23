import 'package:android_adb_tool/android_adb_tool.dart';
import 'package:android_adb_tool/api/tool.dart';
import 'package:flutter/material.dart';

void main() {
  AndroidAdbTool.instance.init();
  runApp(const MyApp());
}

class MyApp extends StatefulWidget {
  const MyApp({super.key});

  @override
  State<MyApp> createState() => _MyAppState();
}

class _MyAppState extends State<MyApp> {
  TextEditingController controller = TextEditingController(text: '10.0.2.2');
  TextEditingController portController = TextEditingController(text: '5037');
  TextEditingController shellController = TextEditingController(text: '');
  String? output;

  AdbDevice? _device;
  @override
  void initState() {
    super.initState();
  }

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(
          title: const Text('Native Packages'),
        ),
        body: SingleChildScrollView(
          child: Container(
            padding: const EdgeInsets.all(10),
            child: Column(
              children: [
                TextField(
                  controller: controller,
                  decoration: const InputDecoration(labelText: 'address'),
                ),
                TextField(
                  controller: portController,
                  decoration: const InputDecoration(labelText: 'port'),
                ),
                ElevatedButton(
                    onPressed: _connect, child: const Text('ADB Connect')),
                TextField(
                  controller: shellController,
                  decoration: const InputDecoration(labelText: 'Run Adb shell'),
                ),
                Wrap(
                  children: [
                    FilledButton(
                        onPressed: _device == null ? null : run,
                        child: const Text('Run Shell Command')),
                    OutlinedButton(
                        onPressed: () async {
                          List<AdbDeviceInfo> devices =
                              await getHost().devices();

                          for (var element in devices) {
                            print(element.data);
                          }
                        },
                        child: const Text("get devices"))
                  ],
                ),
                if (output != null)
                  Card(
                    child: Padding(
                      padding: const EdgeInsets.all(8.0),
                      child: Column(
                        crossAxisAlignment: CrossAxisAlignment.start,
                        children: [
                          Text('输出'),
                          SelectableText(output!),
                        ],
                      ),
                    ),
                  )
              ],
            ),
          ),
        ),
      ),
    );
  }

  DeviceHost getHost() {
    return DeviceHost(
        addr: controller.text, port: int.parse(portController.text));
  }

  Future<void> _connect() async {
    final host = getHost();
    _device = await host.connectToDevice();
    setState(() {});
  }

  Future<void> run() async {
    if (_device != null) {
      final shellCommand = shellController.text;
      try {
        final result =
            await _device!.executeHostShellCommand(shell: shellCommand);
        print(result);
        setState(() {
          output = result;
        });
      } catch (e) {
        print("error ${e}");
      }
    }
  }
}

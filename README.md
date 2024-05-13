# android_adb_tool

adb connect tool 


## Preview

![image](https://raw.githubusercontent.com/mdddj/android_adb_tool/main/img.png)

## API



init `AndroidAdbTool.instance.init();`
```dart
void main() {
  
  //add this line
  AndroidAdbTool.instance.init();
  runApp(const MyApp());
}

```


### 1. connect

```dart
AdbTcpConnection connection =  await AndroidAdbTool.instance.connect("127.0.0.1",5037)
```


### 3. run shell
```dart
Uint8List result = await connection.runShellCommand("df -h");
print("output string:${result.toStringValue}");
```


## Full example

```dart
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
  TextEditingController controller = TextEditingController(text: '127.0.0.1');
  TextEditingController portController = TextEditingController(text: '5037');
  TextEditingController shellController = TextEditingController(text: '');
  String? output;

  AdbTcpConnection? connection;

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
                FilledButton(
                    onPressed: connection == null ? null : run,
                    child: const Text('Run Shell Command')),
                if(output!=null)
                  Card(
                    child: Padding(
                      padding: const EdgeInsets.all(8.0),
                      child: Column(
                        crossAxisAlignment: CrossAxisAlignment.start,
                        children: [
                          Text('输出'),
                          Text(output!),
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

  Future<void> _connect() async {
    connection = await AndroidAdbTool.instance
        .connect(controller.text, int.parse(portController.text));
    setState(() {});
  }

  Future<void> run() async {
    if (connection != null) {
      final shellCommand = shellController.text.split(" ");
      try {
        final result = await connection!.runShellCommand(shellCommand);
        setState(() {
          output = result.toStringValue;
        });
      } catch (e) {
        print("error ${e}");
      }
    }
  }
}

```
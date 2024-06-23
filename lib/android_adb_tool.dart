import 'dart:typed_data';

import 'package:android_adb_tool/frb_generated.dart';

extension Uint8ListEx on Uint8List {
  String get toStringValue {
    String s = String.fromCharCodes(this);
    return s;
  }
}

class AndroidAdbTool {
  AndroidAdbTool._();
  factory AndroidAdbTool() => AndroidAdbTool._();
  static final AndroidAdbTool instance = AndroidAdbTool();

  void init() {
    RustLib.init();
  }
}

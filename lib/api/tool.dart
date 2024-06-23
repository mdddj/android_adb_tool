// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

// These functions are ignored because they are not marked as `pub`: `get_host`
// These function are ignored because they are on traits that is not defined in current crate (put an empty `#[frb]` on it to unignore): `from`

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<AdbDevice>>
abstract class AdbDevice implements RustOpaqueInterface {
  Future<String> executeHostCommand(
      {required String shell, required bool hasOutput, required bool hasLen});

  Future<String> executeHostShellCommand({required String shell});
}

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<DeviceTcpStream>>
abstract class DeviceTcpStream implements RustOpaqueInterface {}

class AdbDeviceInfo {
  final Map<String, String> data;

  const AdbDeviceInfo({
    required this.data,
  });

  @override
  int get hashCode => data.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is AdbDeviceInfo &&
          runtimeType == other.runtimeType &&
          data == other.data;
}

///设备
class DeviceHost {
  final String addr;
  final int port;
  final BigInt? readTimeout;
  final BigInt? writeTimeout;

  const DeviceHost({
    required this.addr,
    required this.port,
    this.readTimeout,
    this.writeTimeout,
  });

  Future<DeviceTcpStream> connect() =>
      RustLib.instance.api.crateApiToolDeviceHostConnect(
        that: this,
      );

  Future<AdbDevice> connectToDevice() =>
      RustLib.instance.api.crateApiToolDeviceHostConnectToDevice(
        that: this,
      );

  Future<List<AdbDeviceInfo>> devices() =>
      RustLib.instance.api.crateApiToolDeviceHostDevices(
        that: this,
      );

  @override
  int get hashCode =>
      addr.hashCode ^
      port.hashCode ^
      readTimeout.hashCode ^
      writeTimeout.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is DeviceHost &&
          runtimeType == other.runtimeType &&
          addr == other.addr &&
          port == other.port &&
          readTimeout == other.readTimeout &&
          writeTimeout == other.writeTimeout;
}

// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0.

#![allow(
    non_camel_case_types,
    unused,
    non_snake_case,
    clippy::needless_return,
    clippy::redundant_closure_call,
    clippy::redundant_closure,
    clippy::useless_conversion,
    clippy::unit_arg,
    clippy::unused_unit,
    clippy::double_parens,
    clippy::let_and_return,
    clippy::too_many_arguments,
    clippy::match_single_binding,
    clippy::clone_on_copy,
    clippy::let_unit_value,
    clippy::deref_addrof,
    clippy::explicit_auto_deref,
    clippy::borrow_deref_ref,
    clippy::needless_borrow
)]

// Section: imports

use crate::api::tool::*;
use flutter_rust_bridge::for_generated::byteorder::{NativeEndian, ReadBytesExt, WriteBytesExt};
use flutter_rust_bridge::for_generated::{transform_result_dco, Lifetimeable, Lockable};
use flutter_rust_bridge::{Handler, IntoIntoDart};

// Section: boilerplate

flutter_rust_bridge::frb_generated_boilerplate!(
    default_stream_sink_codec = SseCodec,
    default_rust_opaque = RustOpaqueMoi,
    default_rust_auto_opaque = RustAutoOpaqueMoi,
);
pub(crate) const FLUTTER_RUST_BRIDGE_CODEGEN_VERSION: &str = "2.0.0";
pub(crate) const FLUTTER_RUST_BRIDGE_CODEGEN_CONTENT_HASH: i32 = -1018463408;

// Section: executor

flutter_rust_bridge::frb_generated_default_handler!();

// Section: wire_funcs

fn wire__crate__api__tool__AdbDevice_execute_host_command_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "AdbDevice_execute_host_command",
            port: Some(port_),
            mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
        },
        move || {
            let message = unsafe {
                flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                    ptr_,
                    rust_vec_len_,
                    data_len_,
                )
            };
            let mut deserializer =
                flutter_rust_bridge::for_generated::SseDeserializer::new(message);
            let api_that = <RustOpaqueMoi<
                flutter_rust_bridge::for_generated::RustAutoOpaqueInner<AdbDevice>,
            >>::sse_decode(&mut deserializer);
            let api_shell = <String>::sse_decode(&mut deserializer);
            let api_has_output = <bool>::sse_decode(&mut deserializer);
            let api_has_len = <bool>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, String>((move || {
                    let mut api_that_guard = None;
                    let decode_indices_ =
                        flutter_rust_bridge::for_generated::lockable_compute_decode_order(vec![
                            flutter_rust_bridge::for_generated::LockableOrderInfo::new(
                                &api_that, 0, false,
                            ),
                        ]);
                    for i in decode_indices_ {
                        match i {
                            0 => api_that_guard = Some(api_that.lockable_decode_sync_ref()),
                            _ => unreachable!(),
                        }
                    }
                    let api_that_guard = api_that_guard.unwrap();
                    let output_ok = crate::api::tool::AdbDevice::execute_host_command(
                        &*api_that_guard,
                        &api_shell,
                        api_has_output,
                        api_has_len,
                    )?;
                    Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__crate__api__tool__AdbDevice_execute_host_shell_command_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "AdbDevice_execute_host_shell_command",
            port: Some(port_),
            mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
        },
        move || {
            let message = unsafe {
                flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                    ptr_,
                    rust_vec_len_,
                    data_len_,
                )
            };
            let mut deserializer =
                flutter_rust_bridge::for_generated::SseDeserializer::new(message);
            let api_that = <RustOpaqueMoi<
                flutter_rust_bridge::for_generated::RustAutoOpaqueInner<AdbDevice>,
            >>::sse_decode(&mut deserializer);
            let api_shell = <String>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, String>((move || {
                    let mut api_that_guard = None;
                    let decode_indices_ =
                        flutter_rust_bridge::for_generated::lockable_compute_decode_order(vec![
                            flutter_rust_bridge::for_generated::LockableOrderInfo::new(
                                &api_that, 0, false,
                            ),
                        ]);
                    for i in decode_indices_ {
                        match i {
                            0 => api_that_guard = Some(api_that.lockable_decode_sync_ref()),
                            _ => unreachable!(),
                        }
                    }
                    let api_that_guard = api_that_guard.unwrap();
                    let output_ok = crate::api::tool::AdbDevice::execute_host_shell_command(
                        &*api_that_guard,
                        &api_shell,
                    )?;
                    Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__crate__api__tool__device_host_connect_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "device_host_connect",
            port: Some(port_),
            mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
        },
        move || {
            let message = unsafe {
                flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                    ptr_,
                    rust_vec_len_,
                    data_len_,
                )
            };
            let mut deserializer =
                flutter_rust_bridge::for_generated::SseDeserializer::new(message);
            let api_that = <crate::api::tool::DeviceHost>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, String>((move || {
                    let output_ok = crate::api::tool::DeviceHost::connect(&api_that)?;
                    Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__crate__api__tool__device_host_connect_to_device_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "device_host_connect_to_device",
            port: Some(port_),
            mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
        },
        move || {
            let message = unsafe {
                flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                    ptr_,
                    rust_vec_len_,
                    data_len_,
                )
            };
            let mut deserializer =
                flutter_rust_bridge::for_generated::SseDeserializer::new(message);
            let api_that = <crate::api::tool::DeviceHost>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, String>((move || {
                    let output_ok = crate::api::tool::DeviceHost::connect_to_device(&api_that)?;
                    Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__crate__api__tool__device_host_devices_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "device_host_devices",
            port: Some(port_),
            mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
        },
        move || {
            let message = unsafe {
                flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                    ptr_,
                    rust_vec_len_,
                    data_len_,
                )
            };
            let mut deserializer =
                flutter_rust_bridge::for_generated::SseDeserializer::new(message);
            let api_that = <crate::api::tool::DeviceHost>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, String>((move || {
                    let output_ok = crate::api::tool::DeviceHost::devices(&api_that)?;
                    Ok(output_ok)
                })())
            }
        },
    )
}

// Section: related_funcs

flutter_rust_bridge::frb_generated_moi_arc_impl_value!(
    flutter_rust_bridge::for_generated::RustAutoOpaqueInner<AdbDevice>
);
flutter_rust_bridge::frb_generated_moi_arc_impl_value!(
    flutter_rust_bridge::for_generated::RustAutoOpaqueInner<DeviceTcpStream>
);

// Section: dart2rust

impl SseDecode for AdbDevice {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut inner = <RustOpaqueMoi<
            flutter_rust_bridge::for_generated::RustAutoOpaqueInner<AdbDevice>,
        >>::sse_decode(deserializer);
        return flutter_rust_bridge::for_generated::rust_auto_opaque_decode_owned(inner);
    }
}

impl SseDecode for DeviceTcpStream {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut inner = <RustOpaqueMoi<
            flutter_rust_bridge::for_generated::RustAutoOpaqueInner<DeviceTcpStream>,
        >>::sse_decode(deserializer);
        return flutter_rust_bridge::for_generated::rust_auto_opaque_decode_owned(inner);
    }
}

impl SseDecode for std::collections::HashMap<String, String> {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut inner = <Vec<(String, String)>>::sse_decode(deserializer);
        return inner.into_iter().collect();
    }
}

impl SseDecode
    for RustOpaqueMoi<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<AdbDevice>>
{
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut inner = <usize>::sse_decode(deserializer);
        return decode_rust_opaque_moi(inner);
    }
}

impl SseDecode
    for RustOpaqueMoi<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<DeviceTcpStream>>
{
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut inner = <usize>::sse_decode(deserializer);
        return decode_rust_opaque_moi(inner);
    }
}

impl SseDecode for String {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut inner = <Vec<u8>>::sse_decode(deserializer);
        return String::from_utf8(inner).unwrap();
    }
}

impl SseDecode for crate::api::tool::AdbDeviceInfo {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut var_data = <std::collections::HashMap<String, String>>::sse_decode(deserializer);
        return crate::api::tool::AdbDeviceInfo { data: var_data };
    }
}

impl SseDecode for bool {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        deserializer.cursor.read_u8().unwrap() != 0
    }
}

impl SseDecode for crate::api::tool::DeviceHost {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut var_addr = <String>::sse_decode(deserializer);
        let mut var_port = <u16>::sse_decode(deserializer);
        let mut var_readTimeout = <Option<u64>>::sse_decode(deserializer);
        let mut var_writeTimeout = <Option<u64>>::sse_decode(deserializer);
        return crate::api::tool::DeviceHost {
            addr: var_addr,
            port: var_port,
            read_timeout: var_readTimeout,
            write_timeout: var_writeTimeout,
        };
    }
}

impl SseDecode for Vec<crate::api::tool::AdbDeviceInfo> {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut len_ = <i32>::sse_decode(deserializer);
        let mut ans_ = vec![];
        for idx_ in 0..len_ {
            ans_.push(<crate::api::tool::AdbDeviceInfo>::sse_decode(deserializer));
        }
        return ans_;
    }
}

impl SseDecode for Vec<u8> {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut len_ = <i32>::sse_decode(deserializer);
        let mut ans_ = vec![];
        for idx_ in 0..len_ {
            ans_.push(<u8>::sse_decode(deserializer));
        }
        return ans_;
    }
}

impl SseDecode for Vec<(String, String)> {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut len_ = <i32>::sse_decode(deserializer);
        let mut ans_ = vec![];
        for idx_ in 0..len_ {
            ans_.push(<(String, String)>::sse_decode(deserializer));
        }
        return ans_;
    }
}

impl SseDecode for Option<u64> {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        if (<bool>::sse_decode(deserializer)) {
            return Some(<u64>::sse_decode(deserializer));
        } else {
            return None;
        }
    }
}

impl SseDecode for (String, String) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut var_field0 = <String>::sse_decode(deserializer);
        let mut var_field1 = <String>::sse_decode(deserializer);
        return (var_field0, var_field1);
    }
}

impl SseDecode for u16 {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        deserializer.cursor.read_u16::<NativeEndian>().unwrap()
    }
}

impl SseDecode for u64 {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        deserializer.cursor.read_u64::<NativeEndian>().unwrap()
    }
}

impl SseDecode for u8 {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        deserializer.cursor.read_u8().unwrap()
    }
}

impl SseDecode for usize {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        deserializer.cursor.read_u64::<NativeEndian>().unwrap() as _
    }
}

impl SseDecode for i32 {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        deserializer.cursor.read_i32::<NativeEndian>().unwrap()
    }
}

fn pde_ffi_dispatcher_primary_impl(
    func_id: i32,
    port: flutter_rust_bridge::for_generated::MessagePort,
    ptr: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len: i32,
    data_len: i32,
) {
    // Codec=Pde (Serialization + dispatch), see doc to use other codecs
    match func_id {
        1 => wire__crate__api__tool__AdbDevice_execute_host_command_impl(
            port,
            ptr,
            rust_vec_len,
            data_len,
        ),
        2 => wire__crate__api__tool__AdbDevice_execute_host_shell_command_impl(
            port,
            ptr,
            rust_vec_len,
            data_len,
        ),
        3 => wire__crate__api__tool__device_host_connect_impl(port, ptr, rust_vec_len, data_len),
        4 => wire__crate__api__tool__device_host_connect_to_device_impl(
            port,
            ptr,
            rust_vec_len,
            data_len,
        ),
        5 => wire__crate__api__tool__device_host_devices_impl(port, ptr, rust_vec_len, data_len),
        _ => unreachable!(),
    }
}

fn pde_ffi_dispatcher_sync_impl(
    func_id: i32,
    ptr: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len: i32,
    data_len: i32,
) -> flutter_rust_bridge::for_generated::WireSyncRust2DartSse {
    // Codec=Pde (Serialization + dispatch), see doc to use other codecs
    match func_id {
        _ => unreachable!(),
    }
}

// Section: rust2dart

// Codec=Dco (DartCObject based), see doc to use other codecs
impl flutter_rust_bridge::IntoDart for FrbWrapper<AdbDevice> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        flutter_rust_bridge::for_generated::rust_auto_opaque_encode::<_, MoiArc<_>>(self.0)
            .into_dart()
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive for FrbWrapper<AdbDevice> {}

impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<AdbDevice>> for AdbDevice {
    fn into_into_dart(self) -> FrbWrapper<AdbDevice> {
        self.into()
    }
}

// Codec=Dco (DartCObject based), see doc to use other codecs
impl flutter_rust_bridge::IntoDart for FrbWrapper<DeviceTcpStream> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        flutter_rust_bridge::for_generated::rust_auto_opaque_encode::<_, MoiArc<_>>(self.0)
            .into_dart()
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive for FrbWrapper<DeviceTcpStream> {}

impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<DeviceTcpStream>> for DeviceTcpStream {
    fn into_into_dart(self) -> FrbWrapper<DeviceTcpStream> {
        self.into()
    }
}

// Codec=Dco (DartCObject based), see doc to use other codecs
impl flutter_rust_bridge::IntoDart for crate::api::tool::AdbDeviceInfo {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        [self.data.into_into_dart().into_dart()].into_dart()
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for crate::api::tool::AdbDeviceInfo
{
}
impl flutter_rust_bridge::IntoIntoDart<crate::api::tool::AdbDeviceInfo>
    for crate::api::tool::AdbDeviceInfo
{
    fn into_into_dart(self) -> crate::api::tool::AdbDeviceInfo {
        self
    }
}
// Codec=Dco (DartCObject based), see doc to use other codecs
impl flutter_rust_bridge::IntoDart for crate::api::tool::DeviceHost {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        [
            self.addr.into_into_dart().into_dart(),
            self.port.into_into_dart().into_dart(),
            self.read_timeout.into_into_dart().into_dart(),
            self.write_timeout.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive for crate::api::tool::DeviceHost {}
impl flutter_rust_bridge::IntoIntoDart<crate::api::tool::DeviceHost>
    for crate::api::tool::DeviceHost
{
    fn into_into_dart(self) -> crate::api::tool::DeviceHost {
        self
    }
}

impl SseEncode for AdbDevice {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <RustOpaqueMoi<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<AdbDevice>>>::sse_encode(flutter_rust_bridge::for_generated::rust_auto_opaque_encode::<_, MoiArc<_>>(self), serializer);
    }
}

impl SseEncode for DeviceTcpStream {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <RustOpaqueMoi<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<DeviceTcpStream>>>::sse_encode(flutter_rust_bridge::for_generated::rust_auto_opaque_encode::<_, MoiArc<_>>(self), serializer);
    }
}

impl SseEncode for std::collections::HashMap<String, String> {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <Vec<(String, String)>>::sse_encode(self.into_iter().collect(), serializer);
    }
}

impl SseEncode
    for RustOpaqueMoi<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<AdbDevice>>
{
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        let (ptr, size) = self.sse_encode_raw();
        <usize>::sse_encode(ptr, serializer);
        <i32>::sse_encode(size, serializer);
    }
}

impl SseEncode
    for RustOpaqueMoi<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<DeviceTcpStream>>
{
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        let (ptr, size) = self.sse_encode_raw();
        <usize>::sse_encode(ptr, serializer);
        <i32>::sse_encode(size, serializer);
    }
}

impl SseEncode for String {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <Vec<u8>>::sse_encode(self.into_bytes(), serializer);
    }
}

impl SseEncode for crate::api::tool::AdbDeviceInfo {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <std::collections::HashMap<String, String>>::sse_encode(self.data, serializer);
    }
}

impl SseEncode for bool {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        serializer.cursor.write_u8(self as _).unwrap();
    }
}

impl SseEncode for crate::api::tool::DeviceHost {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <String>::sse_encode(self.addr, serializer);
        <u16>::sse_encode(self.port, serializer);
        <Option<u64>>::sse_encode(self.read_timeout, serializer);
        <Option<u64>>::sse_encode(self.write_timeout, serializer);
    }
}

impl SseEncode for Vec<crate::api::tool::AdbDeviceInfo> {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <i32>::sse_encode(self.len() as _, serializer);
        for item in self {
            <crate::api::tool::AdbDeviceInfo>::sse_encode(item, serializer);
        }
    }
}

impl SseEncode for Vec<u8> {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <i32>::sse_encode(self.len() as _, serializer);
        for item in self {
            <u8>::sse_encode(item, serializer);
        }
    }
}

impl SseEncode for Vec<(String, String)> {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <i32>::sse_encode(self.len() as _, serializer);
        for item in self {
            <(String, String)>::sse_encode(item, serializer);
        }
    }
}

impl SseEncode for Option<u64> {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <bool>::sse_encode(self.is_some(), serializer);
        if let Some(value) = self {
            <u64>::sse_encode(value, serializer);
        }
    }
}

impl SseEncode for (String, String) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <String>::sse_encode(self.0, serializer);
        <String>::sse_encode(self.1, serializer);
    }
}

impl SseEncode for u16 {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        serializer.cursor.write_u16::<NativeEndian>(self).unwrap();
    }
}

impl SseEncode for u64 {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        serializer.cursor.write_u64::<NativeEndian>(self).unwrap();
    }
}

impl SseEncode for u8 {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        serializer.cursor.write_u8(self).unwrap();
    }
}

impl SseEncode for usize {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        serializer
            .cursor
            .write_u64::<NativeEndian>(self as _)
            .unwrap();
    }
}

impl SseEncode for i32 {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        serializer.cursor.write_i32::<NativeEndian>(self).unwrap();
    }
}

#[cfg(not(target_family = "wasm"))]
#[path = "frb_generated.io.rs"]
mod io;
#[cfg(not(target_family = "wasm"))]
pub use io::*;

/// cbindgen:ignore
#[cfg(target_family = "wasm")]
#[path = "frb_generated.web.rs"]
mod web;
#[cfg(target_family = "wasm")]
pub use web::*;

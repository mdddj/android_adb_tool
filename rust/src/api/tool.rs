use std::{collections::HashMap, net::TcpStream};

use flutter_rust_bridge::frb;
use mozdevice::{Device, DeviceInfo, Host};

///设备
pub struct DeviceHost {
    pub addr: String,
    pub port: u16,
    pub read_timeout: Option<u64>,
    pub write_timeout: Option<u64>,
}

impl DeviceHost {
    fn get_host(&self) -> Host {
        Host {
            host: Some(self.addr.clone()),
            port: Some(self.port),
            ..Default::default()
        }
    }

    #[frb(sync)]
    pub fn connect(&self) -> Result<DeviceTcpStream, String> {
        let r = self.get_host().connect();
        match r {
            Ok(tcp) => Ok(DeviceTcpStream { tcp_stream: tcp }),
            Err(e) => Err(e.to_string()),
        }
    }

    #[frb(sync)]
    pub fn connect_to_device(&self) -> Result<AdbDevice, String> {
        let r = self.get_host().device_or_default(
            Option::<&String>::None,
            mozdevice::AndroidStorageInput::Auto,
        );
        match r {
            Ok(d) => Ok(AdbDevice::from(d)),
            Err(e) => Err(e.to_string()),
        }
    }

    #[frb(sync)]
    pub fn devices(&self) -> Result<Vec<AdbDeviceInfo>, String> {
        let devices: Result<Vec<DeviceInfo>, mozdevice::DeviceError> =
            self.get_host().devices::<Vec<DeviceInfo>>();
        match devices {
            Ok(ds) => Ok(ds
                .iter()
                .map(|v| AdbDeviceInfo {
                    data: v.info.clone().into_iter().collect(),
                })
                .collect()),
            Err(e) => Err(e.to_string()),
        }
    }
}

// impl From<DeviceHost> for Host {
//     fn from(value: DeviceHost) -> Self {
//         Host {
//             host: Some(value.addr),
//             port: Some(value.port),
//             read_timeout: value
//                 .read_timeout
//                 .map_or(Some(Duration::from_secs(2)), |x| {
//                     Some(Duration::from_millis(x))
//                 }),
//             write_timeout: value
//                 .write_timeout
//                 .map_or(Some(Duration::from_secs(2)), |x| {
//                     Some(Duration::from_millis(x))
//                 }),
//         }
//     }
// }

///设备流
pub struct DeviceTcpStream {
    tcp_stream: TcpStream,
}

impl DeviceTcpStream {
    // pub fn read(&mut self, mut buf: Box<[u8]>) -> Result<usize, String> {
    //     let r = self.tcp_stream.read(&mut buf);
    //     match r {
    //         Ok(z) => Ok(z),
    //         Err(e) => Err(e.to_string()),
    //     }
    // }
    // pub fn write(&mut self, buf: Box<[u8]>) -> Result<usize, String> {
    //     let r = self.tcp_stream.write(&buf);
    //     match r {
    //         Ok(size) => Ok(size),
    //         Err(e) => Err(e.to_string()),
    //     }
    // }
}

//设备信息
pub struct AdbDevice {
    device: Device,
}

impl From<Device> for AdbDevice {
    fn from(value: Device) -> Self {
        AdbDevice { device: value }
    }
}

impl AdbDevice {
    #[frb(sync)]
    pub fn execute_host_shell_command(&self, shell: &str) -> Result<String, String> {
        let r = self.device.execute_host_shell_command(shell);
        match r {
            Ok(str) => Ok(str),
            Err(e) => Err(e.to_string()),
        }
    }
    #[frb(sync)]
    pub fn execute_host_command(
        &self,
        shell: &str,
        has_output: bool,
        has_len: bool,
    ) -> Result<String, String> {
        let r = self.device.execute_host_command(shell, has_output, has_len);
        match r {
            Ok(str) => Ok(str),
            Err(e) => Err(e.to_string()),
        }
    }
}

pub struct AdbDeviceInfo {
    pub data: HashMap<String, String>,
}

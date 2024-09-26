use std::{borrow::Borrow, default, process::Command};

extern crate anyhow;
use self::anyhow::Error;
extern crate mozdevice;
use self::mozdevice::{Device, DeviceInfo, Host};
extern crate tracing;
use self::tracing::info;

pub struct AlvrAdb {
    devices: Vec<AlvrDevice>,
}

pub struct AlvrDevice {
    serial: String,
    ports: Vec<u16>,
}

impl AlvrAdb {
    pub fn new() -> Self {
        AlvrAdb { devices: vec![] }
    }
    pub fn forward_ports_to_all(&mut self, local_port: u16, remote_port: u16) -> Result<(), Error> {
        let devices_host = Host::default();
        let connected_devices: Vec<DeviceInfo> = devices_host.devices()?;
        for device in connected_devices {
            let host: mozdevice::Host = Host::default();
            let devices: Device =
                host.device_or_default(Some(&device.serial), mozdevice::AndroidStorageInput::Auto)?;
            let mut device_exists = false;
            let mut device_already_forwarded = false;
            for i in &self.devices {
                if device.serial == i.serial {
                    device_exists = true;
                    break;
                }
            }
            let mut indices_to_remove = vec![];
            for (i, x) in self.devices.iter().enumerate() {
                if !x.serial.contains(&device.serial) {
                    indices_to_remove.push(i);
                }
            }
            for index in indices_to_remove.iter().rev() {
                self.devices.remove(*index);
            }
            if !device_exists {
                self.devices.push(AlvrDevice {
                    serial: device.serial.clone(),
                    ports: vec![],
                });
            } else {
                for devicee in &self.devices {
                    if devicee.serial == device.serial && devicee.ports.contains(&local_port) {
                        device_already_forwarded = true;
                        // so it doesnt spam logs
                        break;
                    }
                }
                break;
            }
            info!("Attempting to forward {}", device.serial);
            match devices.forward_port(local_port, remote_port) {
                Ok(_) => {
                    info!(
                        "Port forwarding successful for {}, port {}",
                        device.serial, local_port
                    );
                    for device in &mut self.devices {
                        if !device.ports.contains(&local_port) {
                            device.ports.push(local_port);
                        }
                    }
                }
                Err(e) => {
                    info!("Failed to forward port for {}: {:?}", device.serial, e);
                }
            }
        }
        Ok(())
    }
}

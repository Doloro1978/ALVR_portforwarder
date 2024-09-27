use std::vec;
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
#[warn(clippy::derivable_impls)]
impl Default for AlvrAdb {
    fn default() -> Self {
        AlvrAdb { devices: vec![] }
    }
}

impl AlvrAdb {
    pub fn forward_ports(&mut self, port: u16) -> Result<(), Error> {
        let devices_host = Host::default();
        let connected_devices: Vec<DeviceInfo> = devices_host.devices()?;
        let mut devices_to_remove: Vec<String> = vec![];
        // if device isnt connected but has been connected before, assume disconected and remove from vec
        self.devices.iter().for_each(|r| {
            if let Some(abc) = connected_devices.iter().find(|x| x.serial == r.serial) {
            } else {
                devices_to_remove.push(r.serial.clone());
            }
        });
        devices_to_remove.iter().for_each(|r| {
            self.devices
                .remove(self.devices.iter().position(|rx| rx.serial == *r).unwrap());
        });

        for device in connected_devices {
            let host: mozdevice::Host = Host::default();
            let devices: Device =
                host.device_or_default(Some(&device.serial), mozdevice::AndroidStorageInput::Auto)?;
            // let mut device_exists = false;
            // let mut device_already_forwarded = false;
            if let Some(device_entry) = self.devices.iter().find(|r| r.serial.eq(&device.serial)) {
                if device_entry.ports.contains(&port) {
                    continue;
                }
            } else {
                self.devices.push(AlvrDevice {
                    serial: device.serial.clone(),
                    ports: vec![],
                });
            }
            info!("Attempting to forward {}", device.serial);
            match devices.forward_port(port, port) {
                Ok(_) => {
                    info!(
                        "Port forwarding successful for {}, port {}",
                        device.serial, port
                    );
                    // add port to entry if it isnt already in there
                    if let Some(wawa) = self
                        .devices
                        .iter_mut()
                        .find(|r| !r.ports.contains(&port) || r.serial.eq(&device.serial))
                    {
                        wawa.ports.push(port);
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

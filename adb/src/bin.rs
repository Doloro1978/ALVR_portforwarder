use std::{thread::sleep, time::Duration};

extern crate tracing;
use tracing::{level_filters::LevelFilter, Level};
extern crate tracing_subscriber;
use tracing_subscriber::EnvFilter;
extern crate alvr_adb;

fn main() {
    // mozdevice RLLY likes to spam
    tracing_subscriber::fmt().init();
    let mut adb_devices = alvr_adb::AlvrAdb::default();
    loop {
        // i should rlly check if any new devices exist by comparing adb_devices
        // to connected devices but egh.. for now this is easy
        adb_devices.forward_ports(9943).unwrap();
        adb_devices.forward_ports(9944).unwrap();
        sleep(Duration::from_secs(2));
    }
}

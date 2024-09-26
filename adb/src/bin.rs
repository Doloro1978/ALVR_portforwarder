use std::{thread::sleep, time::Duration};
mod lib;

fn main() {
    tracing_subscriber::fmt::init();
    // info!("Forwarding all adb");
    let mut adb = lib::AlvrAdb::new();
    loop {
        adb.forward_ports_to_all(9943, 9943).unwrap();
        adb.forward_ports_to_all(9944, 9944).unwrap();
        sleep(Duration::from_secs(2));
    }
}

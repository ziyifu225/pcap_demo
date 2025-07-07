use pcap::{Capture, Device};
use std::str;

fn main() {
    let device_name = "lo";
    let device = Device::list()
        .expect("Device listing failed")
        .into_iter()
        .find(|d| d.name == device_name)
        .expect("Device not found");

    println!("Opening device: {}", device.name);

    let mut cap = Capture::from_device(device)
        .expect("Failed to access device")
        .promisc(true)
        .snaplen(65535)
        .open()
        .expect("Failed to open capture");

    println!("Start capturing...");

    while let Ok(packet) = cap.next_packet() {
        if let Ok(text) = str::from_utf8(packet.data) {
            println!("📥 Received packet: {}", text.trim());
        } else {
            println!("📥 Received non-UTF8 packet ({} bytes)", packet.header.len);
        }
    }
}

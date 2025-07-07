use pcap::{Capture, Device};
use std::fs;

fn main() {
    let payload = fs::read("test-files/hello.txt").expect("Failed to read hello.txt");

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
        .open()
        .expect("Failed to open device");

    cap.sendpacket(payload).expect("Failed to send packet");
    println!("✅ Packet sent from file content!");
}

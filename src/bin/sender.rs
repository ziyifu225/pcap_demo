use pcap::{Capture, Device, Linktype, Packet, PacketHeader};
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};
use libc::timeval;

fn main() {
    let payload = fs::read("test-files/hello.txt").expect("Failed to read hello.txt");
    let cap = Capture::dead(Linktype::ETHERNET).expect("Failed to create dead capture");
    let mut savefile = cap.savefile("hello_world.pcap").expect("Failed to open savefile");

    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let ts = timeval {
        tv_sec: now.as_secs() as i64,
        tv_usec: now.subsec_micros() as i64,
    };

    let header = PacketHeader {
        ts,
        caplen: payload.len() as u32,
        len: payload.len() as u32,
    };

    let packet = Packet::new(&header, &payload);

    savefile.write(&packet);
    println!("✅ Packet written to hello_world.pcap");

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

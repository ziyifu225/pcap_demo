use std::os::unix::net::UnixStream;
use std::io::Write;
use std::fs;
use pcap_demo::usb_packet::{UsbPacket, UsbControlPacket, UsbBulkPacket, UsbInterruptPacket}; 
use bincode;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let socket_path = "/tmp/usb-sim.sock";

    println!("Checking if socket exists: {}", socket_path);
    println!("Exists? {}", std::path::Path::new(socket_path).exists());


    let mut stream = UnixStream::connect(socket_path).expect("Failed to connect to socket");

    let packets = vec![
        UsbPacket::Control(UsbControlPacket {
            request_type: 0x80,
            request: 6,
            value: 0x0100,
            index: 0,
            length: 18,
            data: vec![0xE0],
        }),
        UsbPacket::Bulk(UsbBulkPacket {
            endpoint: 2,
            data: vec![0xDE, 0xAD, 0xBE, 0xEF],
        }),
        UsbPacket::Interrupt(UsbInterruptPacket {
            endpoint: 1,
            interval: 10,
            data: vec![0xA5],
        }),
    ];

    for packet in packets {
        let encoded = bincode::serialize(&packet).expect("Failed to serialize");
        stream.write_all(&encoded).expect("Failed to send");
        println!("✅ Sent: {:?}", packet);

        sleep(Duration::from_millis(500));
    }
}

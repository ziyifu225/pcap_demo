use std::os::unix::net::UnixStream;
use std::io::Write;
use std::fs;
use pcap_demo::usb_packet::UsbControlPacket; 
use bincode;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let socket_path = "/tmp/usb-sim.sock";

    println!("Checking if socket exists: {}", socket_path);
    println!("Exists? {}", std::path::Path::new(socket_path).exists());


    let mut stream = UnixStream::connect(socket_path).expect("Failed to connect to socket");

    let packets = vec![
        UsbControlPacket {
            request_type: 0x80, // IN, Standard, Device
            request: 6,
            value: 0x0100,
            index: 0,
            length: 18,
            data: vec![0xE0],
        },
        UsbControlPacket {
            request_type: 0x00, // OUT, Standard, Device
            request: 9, // SET_CONFIGURATION
            value: 1,
            index: 0,
            length: 0,
            data: vec![0xF0],
        },
        UsbControlPacket {
            request_type: 0x21, // OUT, Class, Interface
            request: 0x09, // SET_REPORT
            value: 0x0200,
            index: 0x01,
            length: 2,
            data: vec![0xAB, 0xCD],
        },
    ];

    for packet in packets {
        let encoded = bincode::serialize(&packet).expect("Failed to serialize");
        stream.write_all(&encoded).expect("Failed to send");
        println!("✅ Sent: {:?}", packet);

        sleep(Duration::from_millis(500));
    }
}

use std::os::unix::net::UnixStream;
use std::io::Write;
use std::fs;
use pcap_demo::usb_packet::UsbLikePacket; 
use bincode;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let socket_path = "/tmp/usb-sim.sock";

    println!("Checking if socket exists: {}", socket_path);
    println!("Exists? {}", std::path::Path::new(socket_path).exists());


    let mut stream = UnixStream::connect(socket_path).expect("Failed to connect to socket");

    let packets = vec![
        UsbLikePacket {
            endpoint: 1,
            direction: 0,
            payload_len: 5,
            payload: b"hello".to_vec(),
        },
        UsbLikePacket {
            endpoint: 2,
            direction: 1,
            payload_len: 6,
            payload: b"world!".to_vec(),
        },
        UsbLikePacket {
            endpoint: 3,
            direction: 0,
            payload_len: 4,
            payload: b"data".to_vec(),
        },
    ];

    for packet in packets {
        let encoded = bincode::serialize(&packet).expect("Failed to serialize");
        stream.write_all(&encoded).expect("Failed to send");
        println!("✅ Sent: {:?}", packet);

        sleep(Duration::from_millis(500));
    }
}

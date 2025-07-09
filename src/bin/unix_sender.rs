use std::os::unix::net::UnixStream;
use std::io::Write;
use std::fs;
use pcap_demo::usb_packet::UsbLikePacket; 
use bincode;

fn main() {
    let socket_path = "/tmp/usb-sim.sock";

    println!("Checking if socket exists: {}", socket_path);
    println!("Exists? {}", std::path::Path::new(socket_path).exists());


    let mut stream = UnixStream::connect(socket_path).expect("Failed to connect to socket");

    let message = UsbLikePacket {
        endpoint: 1,
        direction: 0,
        payload_len: 5,
        payload: b"hello".to_vec(),
    };

    let encoded = bincode::serialize(&message).expect("Failed to serialize");
    stream.write_all(&encoded).expect("Failed to send");
    println!("Sent.");
}

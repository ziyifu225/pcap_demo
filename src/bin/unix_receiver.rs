use std::os::unix::net::UnixListener;
use std::io::Read;
use std::fs;
use pcap_demo::usb_packet::UsbLikePacket;
use bincode;

use pcap::{Capture, Linktype, Packet, PacketHeader};
use std::time::{SystemTime, UNIX_EPOCH};
use libc::timeval;

fn main() {
    let socket_path = "/tmp/usb-sim.sock";
    if fs::metadata(socket_path).is_ok() {
        let _ = fs::remove_file(socket_path);
    }

    // initialize PCAP
    let cap = Capture::dead(Linktype::ETHERNET).expect("Failed to create dead capture");
    let mut savefile = cap.savefile("usb_sim_output.pcap").expect("Failed to open savefile");

    let listener = UnixListener::bind(socket_path).expect("Failed to bind");
    println!("Listening on socket: {}", socket_path);

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buf = vec![0u8; 4096];
                let len = stream.read(&mut buf).expect("Failed to read");

                let packet: UsbLikePacket =
                    bincode::deserialize(&buf[..len]).expect("Failed to decode");
                println!("Received: {:?}", packet);

                // Verify payload length
                if packet.payload.len() != packet.payload_len as usize {
                    eprintln!(
                        "Payload length mismatch: declared {}, actual {}",
                        packet.payload_len,
                        packet.payload.len()
                    );
                    continue;
                }
                println!("payload length matches. Processing...");

                // time stamp
                let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
                let ts = timeval {
                    tv_sec: now.as_secs() as i64,
                    tv_usec: now.subsec_micros() as i64,
                };
                let header = PacketHeader {
                    ts,
                    caplen: packet.payload.len() as u32,
                    len: packet.payload.len() as u32,
                };
                let packet = Packet {
                    header: &header,
                    data: &packet.payload,
                };
                savefile.write(&packet);
                savefile.flush().expect("Flush failed");
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}

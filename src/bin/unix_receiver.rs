use std::os::unix::net::UnixListener;
use std::io::Read;
use std::fs;
use pcap_demo::usb_packet::UsbPacket;

use pcap::{Capture, Linktype};
use std::time::{SystemTime, UNIX_EPOCH};
use libc::timeval;
use pcap_demo::handler::handle_usb_packet;

fn main() {
    let socket_path = "/tmp/usb-sim.sock";
    if fs::metadata(socket_path).is_ok() {
        let _ = fs::remove_file(socket_path);
    }

    // initialize PCAP
    let cap = Capture::dead(Linktype::USER0).expect("Failed to create dead capture");
    let mut savefile = cap.savefile("usb_sim_output.pcap").expect("Failed to open savefile");

    let listener = UnixListener::bind(socket_path).expect("Failed to bind");
    println!("Listening on socket: {}", socket_path);

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buf = vec![0u8; 4096];
                loop{
                    match stream.read(&mut buf){
                        Ok(0) => break,
                        Ok(len) => {
                            match bincode::deserialize::<UsbPacket>(&buf[..len]) {
                                Ok(packet) => handle_usb_packet(packet, &mut savefile),
                                Err(e) => eprintln!("Failed to decode packet: {}", e),
                            }
                        }
                        Err(e) => {
                            eprintln!("Read error: {}", e);
                            break;
                        }
                    }
                }
            }  
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}

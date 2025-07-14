use std::os::unix::net::UnixListener;
use std::fs;
use std::io::BufReader;
use pcap_demo::usb_packet::*;
use pcap::{Capture, Linktype};
use pcap_demo::handler::handle_usb_packet;

fn main() {
    let socket_path = "/tmp/usb-sim.sock";
    if fs::metadata(socket_path).is_ok() {
        let _ = fs::remove_file(socket_path);
    }

    let cap = Capture::dead(Linktype::USER0).expect("Failed to create dead capture");
    let mut savefile = cap.savefile("usb_sim_output.pcap").expect("Failed to open savefile");

    let listener = UnixListener::bind(socket_path).expect("Failed to bind");
    println!("Listening on socket: {}", socket_path);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let mut reader = BufReader::new(stream);
                loop {
                    match bincode::deserialize_from::<_, UsbPacketEnvelope>(&mut reader) {
                        Ok(envelope) => {
                            handle_usb_packet(envelope, &mut savefile);
                        }
                        Err(e) => {
                            if let bincode::ErrorKind::Io(ref io_err) = *e {
                                if io_err.kind() == std::io::ErrorKind::UnexpectedEof {
                                    println!("End of stream reached.");
                                    break;
                                }
                            }
                            eprintln!("Failed to decode: {}", e);
                        }
                    }
                }
            }
            Err(e) => eprintln!("Connection error: {}", e),
        }
    }
}

use std::net::UdpSocket;
use pcap::{Capture, Device, Linktype, Packet, PacketHeader};
use std::time::{SystemTime, UNIX_EPOCH};
use libc::timeval;

fn main() {
    let socket = UdpSocket::bind("127.0.0.1:7878").expect("Failed to bind receiver socket");
    println!("Receiver listening on 127.0.0.1:7878");

    let cap = Capture::dead(Linktype::ETHERNET).expect("Failed to create dead capture");
    let mut savefile = cap.savefile("usb_like_output.pcap").expect("Failed to open savefile");

    let mut buf = [0u8; 4096];
    loop {
        match socket.recv_from(&mut buf){
            Ok((len, src)) => {
                let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
                let ts = timeval {
                    tv_sec: now.as_secs() as i64,
                    tv_usec: now.subsec_micros() as i64,
                };
                println!("Received {} bytes from {}", len, src);
                println!("Content:\n{}", String::from_utf8_lossy(&buf[..len]));
                let header = PacketHeader {
                    ts,
                    caplen: len as u32,
                    len: len as u32,
                };
                let packet = Packet {
                    header: &header,
                    data: &buf[..len],
                };
                savefile.write(&packet);
                savefile.flush().expect("Flush failed");
            }
            Err(e) => {
                eprintln!("Error receiving: {}", e);
                break;
            }       
        }
    }
}

use crate::usb_packet::UsbControlPacket;
use pcap::{Packet, PacketHeader, Savefile};
use std::time::{SystemTime, UNIX_EPOCH};
use libc::timeval;

pub fn handle_usb_packet(packet: UsbControlPacket, savefile: &mut Savefile) {
    println!("Received: {:?}", packet);

    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let ts = timeval {
        tv_sec: now.as_secs() as i64,
        tv_usec: now.subsec_micros() as i64,
    };

    let header = PacketHeader {
        ts,
        caplen: packet.data.len() as u32,
        len: packet.data.len() as u32,
    };

    let data_packet = Packet {
        header: &header,
        data: &packet.data,
    };

    savefile.write(&data_packet);
    savefile.flush().expect("Flush failed");
}
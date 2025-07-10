use crate::usb_packet::UsbPacket;
use pcap::{Packet, PacketHeader, Savefile};
use std::time::{SystemTime, UNIX_EPOCH};
use libc::timeval;

pub fn handle_usb_packet(packet: UsbPacket, savefile: &mut Savefile) {
    println!("Received: {:?}", packet);

    let (bytes_to_write, len_info) = match &packet {
        UsbPacket::Control(ctrl) => (ctrl.data.as_slice(), ctrl.data.len()),
        UsbPacket::Bulk(bulk) => (bulk.data.as_slice(), bulk.data.len()),
        UsbPacket::Interrupt(interrupt) => (interrupt.data.as_slice(), interrupt.data.len()),
    };

    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let ts = timeval {
        tv_sec: now.as_secs() as i64,
        tv_usec: now.subsec_micros() as i64,
    };

    let header = PacketHeader {
        ts,
        caplen: len_info as u32,
        len: len_info as u32,
    };

    let data_packet = Packet {
        header: &header,
        data: &bytes_to_write,
    };

    savefile.write(&data_packet);
    savefile.flush().expect("Flush failed");
}
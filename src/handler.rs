use crate::usb_packet::*;
use pcap::{Packet, PacketHeader, Savefile};
use std::time::{SystemTime, UNIX_EPOCH};
use libc::timeval;

pub fn handle_usb_packet(envelope: UsbPacketEnvelope, savefile: &mut Savefile) {
    println!("Received packet_id {}: {:?}", envelope.packet_id, envelope.payload);

    let (data, length) = match &envelope.payload {
        UsbPacket::Control(stage) => match stage {
            ControlStage::Setup(_) => (&b"SETUP"[..], 5),
            ControlStage::Data(data) => (&data[..], data.len()),
            ControlStage::StatusAck => (&b"ACK"[..], 3),
        },
        UsbPacket::Bulk(bulk) => (&bulk.data[..], bulk.data.len()),
        UsbPacket::Interrupt(interrupt) => (&interrupt.data[..], interrupt.data.len()),
    };

    // time stamp
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let ts = timeval {
        tv_sec: now.as_secs() as i64,
        tv_usec: now.subsec_micros() as i64,
    };

    let header = PacketHeader {
        ts,
        caplen: length as u32,
        len: length as u32,
    };

    let data_packet = Packet {
        header: &header,
        data,
    };

    savefile.write(&data_packet);
    savefile.flush().expect("Flush failed");
}
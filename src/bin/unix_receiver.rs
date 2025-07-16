use std::os::unix::net::UnixListener;
use std::fs;
use std::io::{BufReader, Read};
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
        let mut reader = BufReader::new(stream.expect("stream error"));

        loop {
            match read_packet(&mut reader) {
                Some(envelope) => handle_usb_packet(envelope, &mut savefile),
                None => break,
            }
        }
    }
}

fn read_packet(reader: &mut impl Read) -> Option<UsbPacketEnvelope> {
    let mut id_buf = [0u8; 8];
    reader.read_exact(&mut id_buf).ok()?;
    let packet_id = u64::from_le_bytes(id_buf);

    let mut type_buf = [0u8; 1];
    reader.read_exact(&mut type_buf).ok()?;

    let payload = match type_buf[0] {
        0x01 => {
            let mut ctrl_buf = [0u8; 8];
            reader.read_exact(&mut ctrl_buf).ok()?;
            let request_type = ctrl_buf[0];
            let request = ctrl_buf[1];
            let value = u16::from_le_bytes([ctrl_buf[2], ctrl_buf[3]]);
            let index = u16::from_le_bytes([ctrl_buf[4], ctrl_buf[5]]);
            let length = u16::from_le_bytes([ctrl_buf[6], ctrl_buf[7]]);

            let mut data = vec![0u8; length as usize];
            reader.read_exact(&mut data).ok()?;

            UsbPacket::Control(ControlStage::Setup(UsbControlPacket {
                request_type,
                request,
                value,
                index,
                length,
                data,
            }))
        }
        0x02 => {
            let mut data = vec![0u8; 5];
            reader.read_exact(&mut data).ok()?;
            UsbPacket::Control(ControlStage::Data(data))
        }
        0x03 => UsbPacket::Control(ControlStage::StatusAck),
        0x04 => {
            let mut ep_buf = [0u8; 1];
            reader.read_exact(&mut ep_buf).ok()?;
            let mut data = vec![0u8; 7];
            reader.read_exact(&mut data).ok()?;
            UsbPacket::Bulk(UsbBulkPacket {
                endpoint: ep_buf[0],
                data,
            })
        }
        0x05 => {
            let mut meta = [0u8; 2];
            reader.read_exact(&mut meta).ok()?;
            let endpoint = meta[0];
            let interval = meta[1];
            let mut data = vec![0u8; 1];
            reader.read_exact(&mut data).ok()?;
            UsbPacket::Interrupt(UsbInterruptPacket {
                endpoint,
                interval,
                data,
            })
        }
        _ => return None,
    };

    Some(UsbPacketEnvelope {
        packet_id,
        payload,
    })
}

use std::os::unix::net::UnixStream;
use std::io::Write;
use pcap_demo::usb_packet::*; 
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let socket_path = "/tmp/usb-sim.sock";

    println!("Checking if socket exists: {}", socket_path);
    println!("Exists? {}", std::path::Path::new(socket_path).exists());

    let mut stream = UnixStream::connect(socket_path).expect("Failed to connect to socket");

    let control_packet = UsbControlPacket {
        request_type: 0x80,
        request: 6,
        value: 0x0100,
        index: 0,
        length: 18,
        data: vec![0x00; 18],
    };

    let packets = vec![
        UsbPacketEnvelope {
            packet_id: 1,
            payload: UsbPacket::Control(ControlStage::Setup(control_packet)),
        },
        UsbPacketEnvelope {
            packet_id: 1,
            payload: UsbPacket::Control(ControlStage::Data(vec![0xAA, 0xBB, 0xCC, 0xDD, 0xEE])),
        },
        UsbPacketEnvelope {
            packet_id: 1,
            payload: UsbPacket::Control(ControlStage::StatusAck),
        },
        UsbPacketEnvelope {
            packet_id: 2,
            payload: UsbPacket::Bulk(UsbBulkPacket {
                endpoint: 2,
                data: vec![0xDE, 0xAD, 0xBE, 0xEF, 0xAB, 0xFA, 0xCD],
            }),      
        },
        UsbPacketEnvelope {
            packet_id: 3,
            payload: UsbPacket::Interrupt(UsbInterruptPacket {
                endpoint: 1,
                interval: 10,
                data: vec![0xA5],
            }),      
        },
    ];

    for packet in packets {
        let mut encoded = Vec::new();
        encoded.extend(&packet.packet_id.to_le_bytes());

        match &packet.payload {
            UsbPacket::Control(stage) => match stage {
                ControlStage::Setup(setup) => {
                    encoded.push(0x01);
                    encoded.extend(setup.to_bytes());
                }
                ControlStage::Data(data) => {
                    encoded.push(0x02);
                    encoded.extend(data);
                }
                ControlStage::StatusAck => {
                    encoded.push(0x03);
                }
            },
            UsbPacket::Bulk(bulk) => {
                encoded.push(0x04);
                encoded.push(bulk.endpoint);
                encoded.extend(&bulk.data);
            }
            UsbPacket::Interrupt(interrupt) => {
                encoded.push(0x05);
                encoded.push(interrupt.endpoint);
                encoded.push(interrupt.interval);
                encoded.extend(&interrupt.data);
            }
        }

        for chunk in encoded.chunks(8) {
            stream.write_all(chunk).expect("Failed to send chunk");
            println!("Sent chunk ({} bytes): {:?}", chunk.len(), chunk);
            sleep(Duration::from_millis(20));
        }
        println!("Sent full packet: {:?}", packet);
        sleep(Duration::from_millis(500));
    }
}

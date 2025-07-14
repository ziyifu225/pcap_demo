use std::os::unix::net::UnixStream;
use std::io::Write;
use pcap_demo::usb_packet::*; 
use bincode;
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
                data: vec![0xDE, 0xAD, 0xBE, 0xEF],
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
        let encoded = bincode::serialize(&packet).expect("Failed to serialize");

        let chunk_size = 8;
        for chunk in encoded.chunks(chunk_size) {
            stream.write_all(chunk).expect("Failed to send chunk");
            println!("Sent chunk ({} bytes)", chunk.len());
            sleep(Duration::from_millis(20));
        }
        println!("Sent full packet: {:?}", packet);
        sleep(Duration::from_millis(500));
    }
}

use std::fs;
use std::net::UdpSocket;

fn main() {
    let content = fs::read("test-files/udp_test.txt").expect("Failed to read file");

    let socket = UdpSocket::bind("127.0.0.1:3400").expect("Failed to bind sender socket");
    socket
        .send_to(&content, "127.0.0.1:7878")
        .expect("Failed to send data");

    println!("✅ Sender: sent {} bytes", content.len());
}

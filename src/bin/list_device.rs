use pcap::Device;

fn main() {
    let devices = Device::list().expect("Failed to list devices");

    println!("Available devices:");
    for dev in devices {
        println!("- {}", dev.name);
    }
}

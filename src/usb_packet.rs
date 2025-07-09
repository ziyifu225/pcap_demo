use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UsbControlPacket {
    pub request_type: u8,   // bmRequestType
    pub request: u8,        // bRequest
    pub value: u16,         // wValue
    pub index: u16,         // wIndex
    pub length: u16,        // wLength
    pub data: Vec<u8>,      // payload
}

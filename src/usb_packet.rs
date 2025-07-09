use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UsbLikePacket {
    pub endpoint: u8,
    pub direction: u8,
    pub payload_len: u16,
    pub payload: Vec<u8>,
}

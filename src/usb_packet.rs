#[derive(Debug)]
pub struct UsbPacketEnvelope {
    pub packet_id: u64,
    pub payload: UsbPacket,
}

#[derive(Debug)]
pub enum UsbPacket {
    Control(ControlStage),
    Bulk(UsbBulkPacket),
    Interrupt(UsbInterruptPacket),
}

#[derive(Debug)]
pub enum ControlStage {
    Setup(UsbControlPacket),
    Data(Vec<u8>),
    StatusAck,
}

#[derive(Debug)]
pub struct UsbControlPacket {
    pub request_type: u8,   
    pub request: u8,   
    pub value: u16,  
    pub index: u16, 
    pub length: u16,  
    pub data: Vec<u8>,  
}

#[derive(Debug)]
pub struct UsbBulkPacket {
    pub endpoint: u8,
    pub data: Vec<u8>,
}

#[derive(Debug)]
pub struct UsbInterruptPacket {
    pub endpoint: u8,
    pub interval: u8,
    pub data: Vec<u8>,
}
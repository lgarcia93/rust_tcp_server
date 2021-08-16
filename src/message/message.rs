use std::net::{TcpStream};
use std::io::Write;
use std::ops::Deref;


#[derive(Debug)]
pub enum MessageType {
    ListFiles,
    GetFile,
    Unknown,
}

pub struct MessageHeader {
    pub message_type: MessageType,
    pub content_size: u32,
}

pub struct Message {
    pub header: MessageHeader,
    pub content: Vec<u8>,
}

impl MessageHeader {
    fn deserialize(data: Vec<u8>) -> MessageHeader {
        MessageHeader {
            message_type: MessageType::deserialize(data[1]),
            content_size: decode_int32(&data[2..6]),
        }
    }

    fn serialize(&self) -> Vec<u8> {
        let mut serialized_message_header = vec![
            0x7C as u8,
            self.message_type.serialize(),
        ];

        println!("Message type{}", self.message_type.serialize());
        // Content size
        serialized_message_header.extend_from_slice(&serialize_message_size(self.content_size));

        serialized_message_header.push(0x7C);

        serialized_message_header
    }
}

impl Message {
    fn serialize(&mut self) -> Vec<u8> {
        let mut message = Vec::new();

        message.push(0x1B as u8);

        message.append(&mut self.header.serialize());

        message.append(&mut self.content);

        message.push(0x1B);

        message
    }

    pub fn deserialize(data: Vec<u8>) -> Message {
        let header_slice = &data[1..8];

        Message{
            header: MessageHeader::deserialize(header_slice.to_vec()),
            content: data[8..data.len() -1].to_vec(),
        }
    }

    fn message_size(&self) -> usize {
        // 7 bytes header
        // 2 bytes of escape char
        // content-size bytes
        9 + self.header.content_size as usize
    }

    pub fn decode_content_as_string(&self) -> String {
        String::from(String::from_utf8_lossy(&self.content).deref())
    }

    pub fn send(&mut self, stream: &mut TcpStream) {
        let mut written_bytes_count: usize = 0;

        let serialized_message = self.serialize();

        println!("Serialized message: {:?}", serialized_message);

        let message = serialized_message.as_slice();

        while match stream.write(&message[written_bytes_count..self.message_size()]) {
            Ok(written_bytes) => {
                if written_bytes > 0 {
                    written_bytes_count += written_bytes;

                    true
                } else {
                    false
                }
            },
            Err(_) => {
                false
            }
        } {}

        println!("Message sent");
    }
}


impl MessageType {
    fn deserialize(value: u8) -> MessageType {
        println!("Deserialize message type {}", value);
        match value {
            1 => MessageType::ListFiles,
            2 => MessageType::GetFile,
            _ => MessageType::Unknown
        }
    }

    fn serialize(&self) -> u8 {
        match self {
            Self::ListFiles => 1,
            Self::GetFile => 2,
            Self::Unknown => 3,
        }
    }
}

// Transform u32 into 4-byte slice
fn serialize_message_size(x: u32) -> [u8; 4] {
    let b1: u8 = ((x >> 24) & 0xff) as u8;
    let b2: u8 = ((x >> 16) & 0xff) as u8;
    let b3: u8 = ((x >> 8) & 0xff) as u8;
    let b4: u8 = (x & 0xff) as u8;
    return [b1, b2, b3, b4];
}

fn decode_int32(int_bytes: &[u8]) -> u32 {
    return ((int_bytes[0] as u32) << 24) + ((int_bytes[1] as u32) << 16) + ((int_bytes[2] as u32) << 8) + (int_bytes[3]) as u32;
}
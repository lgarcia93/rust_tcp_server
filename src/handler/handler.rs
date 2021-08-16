use std::net::{TcpStream, Shutdown};
use std::io::{Read};
use std::fs::{File};
pub use crate::message::message::{MessageType, MessageHeader, Message};
use chrono::Local;
use std::env

fn get_file_content(file_name: &str) -> Vec<u8> {
    let base_dir = env::var("serverdir")

    let file = File::open(format!("{}/{}", base_dir, file_name));

    let mut buffer = [0; 50];

    let mut file_bytes = Vec::new();

    if let Ok(mut file) = file {
        while match file.read(&mut buffer) {
            Ok(size) => {
                if size > 0 {
                    file_bytes.extend_from_slice(&buffer);
                    true
                } else {
                    false
                }
            },
            Err(err) => {
                println!("Error reading file {}", err);
                false
            }
        } {}
    } else {
        println!("{:?}", file);
    }

    return file_bytes;
}

pub fn process_complete_message(stream: &mut TcpStream, message: &Vec<u8>) {

    let received_message = Message::deserialize(message.to_vec());

    match received_message.header.message_type {
        MessageType::ListFiles => {
            println!("{}: List files message", Local::now());
        },
        MessageType::GetFile => {

            println!("{}: Message is of GetFile Type.", Local::now());

            let file_content = get_file_content(&received_message.decode_content_as_string());

            let mut message = Message {
              header: MessageHeader {
                  message_type: MessageType::GetFile,
                  content_size: file_content.len() as u32
              },
              content: file_content,
            };

            println!("{}: Sending content to client.", Local::now());

            message.send(stream);

        },
        MessageType::Unknown => {
            println!("{}: Unkown message", Local::now());
        }
    }
}

pub fn handle_client_connection(mut stream: TcpStream) {

    let mut buffer = [0 as u8; 32]; //32 byte buffer

    let mut data: Vec<u8> = Vec::new();

    let mut found_escape_char_count = 0;

    println!("{}: Starting stream reading.", Local::now());

    while match stream.read(&mut buffer) {
       Ok(read_bytes) => {

           for n in 0..read_bytes {
               // ESC
                if buffer[n] == 0x1B {
                    found_escape_char_count += 1;
                }

                data.push(buffer[n]);

               if found_escape_char_count == 2 {
                   found_escape_char_count = 0;

                   println!("{}: Found complete message.", Local::now());

                   process_complete_message(&mut stream, &data);

                   data.clear();
               }
           }

           true
       },
       Err(_) => {
           println!("Error reading stream");
           let _ = stream.shutdown(Shutdown::Both);
           false
       }
    } {}
}
extern crate chrono;
use std::net::{TcpListener};
use std::thread;
use chrono::{Local, DateTime};

mod message;
mod default_handler;
mod handler;


fn main() {
    println!("{}: Starting server", Local::now());

    let listener = TcpListener::bind("0.0.0.0:3333").expect("Error binding to Address");

    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            thread::spawn(move || {               
                handler::handler::handle_client_connection(stream);
            });
        } else {
            println!("Invalid stream");
        }
    }

    drop(listener);
}

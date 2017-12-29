extern crate cursive;

use std::io::{Write, BufReader, BufRead, Read};
use std::net::TcpStream;
use std::error::Error;

pub mod client;
pub mod server;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

const MAX_REQUEST_SIZE: u64 = 1048576; // 1Mb

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub msg_type: MsgType,
    pub contents: String,
}

impl Message {
    pub fn new_msg_string(msg_type: MsgType, contents: String) -> Result<String, Box<Error>> {
        Ok(serde_json::to_string(&Message {
            msg_type: msg_type,
            contents: contents,
        })?)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginRequest {
    pub name: String,
}

impl LoginRequest {
    pub fn new_msg_string(name: String) -> Result<String, Box<Error>> {
        Message::new_msg_string(
            MsgType::LoginRequest,
            serde_json::to_string(&LoginRequest { name: name })?,
        )
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginResponse {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Status {
    pub error: bool,
}

impl Status {
    pub fn new_msg_string(error: bool) -> Result<String, Box<Error>> {
        Message::new_msg_string(
            MsgType::Status,
            serde_json::to_string(&Status { error: error })?,
        )
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ChatMessage {
    pub text: String,
    pub author: String,
}

impl ChatMessage {
    pub fn new_msg_string(text: String, author: String) -> Result<String, Box<Error>> {
        Message::new_msg_string(
            MsgType::ChatMessage,
            serde_json::to_string(&ChatMessage {
                text: text,
                author: author,
            })?,
        )
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum MsgType {
    LoginRequest,
    LoginResponse,
    ChatMessage,
    Status,
}

pub fn write_line(stream: &mut TcpStream, contents: String) -> Result<(), Box<Error>> {
    stream.write((contents + "\n").as_bytes())?;
    Ok(())
}

pub fn read_line(stream: &TcpStream) -> Result<String, Box<Error>> {
    let mut buffer = String::new();
    BufReader::new(stream).take(MAX_REQUEST_SIZE).read_line(
        &mut buffer,
    )?;
    Ok(buffer)
}

#[cfg(test)]
mod tests {
    use std::net::{TcpStream, TcpListener};
    use std::thread;
    use super::*;

    #[test]
    fn test_read_write_line() {
        let addr = "127.0.0.1:8846";
        let listener = TcpListener::bind(addr).unwrap();

        let write_is_ok = thread::spawn(move || {
            let mut stream = TcpStream::connect(addr).unwrap();
            write_line(&mut stream, "test".into()).is_ok()
        });

        let stream = listener.accept().unwrap().0;
        let line = read_line(&stream).unwrap();
        assert_eq!(line, "test\n");
        assert!(write_is_ok.join().unwrap());
    }
}

use std::env;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::process;
use std::sync::{mpsc, Mutex, Arc};
use std::thread;
use std::error::Error;
use std::collections::HashMap;

use super::{ChatMessage, Message, MsgType, LoginRequest, Status, read_line, write_line};

extern crate serde;
extern crate serde_json;

type Clients = Arc<Mutex<HashMap<u32, mpsc::Sender<ChatMessage>>>>;

pub fn run_server() {
    let args: Vec<_> = env::args().collect();

    if args.len() != 3 {
        println!("Usage: chat-server (addr)");
        process::exit(1);
    }

    server(args[2].to_string());
}

fn server(addr: String) {
    let listener = TcpListener::bind(&addr).unwrap();
    println!("Listening on addr: {}", addr);

    let clients = Arc::new(Mutex::new(HashMap::new()));
    let mut client_counter = 0;
    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            let (sender, receiver) = mpsc::channel();

            let mut handle = clients.lock().unwrap();
            handle.insert(client_counter, sender);

            let clients_clone = clients.clone();
            thread::spawn(move || if let Err(error) = handle_request(
                stream,
                receiver,
                clients_clone,
                client_counter,
            )
            {
                println!("Client error: {:?}", error);
            });

            client_counter += 1;
        } else if let Err(error) = stream {
            println!("Client error: {:?}", error);
        }
    }
}

fn handle_request(
    stream: TcpStream,
    receiver: mpsc::Receiver<ChatMessage>,
    clients: Clients,
    client_id: u32,
) -> Result<(), Box<Error>> {
    let line = read_line(&stream)?;

    let message: Message = serde_json::from_str(&line)?;
    println!("{:?}", message);

    let clients_clone = clients.clone();
    let resp = match message.msg_type {
        MsgType::LoginRequest => handle_login(serde_json::from_str(message.contents.as_ref())?)?,
        MsgType::ChatMessage => {
            handle_chat_message(
                clients_clone,
                serde_json::from_str(message.contents.as_ref())?,
            )?
        }
        _ => "".into(),
    };

    let mut stream = stream;
    write_line(&mut stream, resp).unwrap();

    if message.msg_type == MsgType::LoginRequest {
        loop {
            let msg = receiver.recv()?;
            let msg_str =
                Message::new_msg_string(MsgType::ChatMessage, serde_json::to_string(&msg)?)?;
            stream.write(msg_str.as_bytes())?;
            stream.write("\n".as_bytes())?;
        }
    }

    // remove self from clients after finished
    let mut handle = clients.lock().unwrap();
    handle.remove(&client_id);

    Ok(())
}

fn handle_chat_message(clients: Clients, chat_message: ChatMessage) -> Result<String, Box<Error>> {
    let handle = clients.lock().unwrap();
    for client in handle.values() {
        client.send(chat_message.clone()).unwrap_or(());
    }
    Status::new_msg_string(false)
}

fn handle_login(login: LoginRequest) -> Result<String, Box<Error>> {
    let _ = login;
    Status::new_msg_string(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn _make_clients(
        s: mpsc::Sender<ChatMessage>,
    ) -> Arc<Mutex<HashMap<u32, mpsc::Sender<ChatMessage>>>> {
        let clients = Arc::new(Mutex::new(HashMap::new()));
        {
            let mut handle = clients.lock().unwrap();
            handle.insert(0, s);
        }

        clients
    }

    #[test]
    fn test_handle_chat_message() {
        let (tx, rx) = mpsc::channel();
        let chat_message = ChatMessage {
            text: "Hello".into(),
            author: "Ben".into(),
        };

        // handle chat message
        let clients = _make_clients(tx);
        handle_chat_message(clients, chat_message.clone()).unwrap();

        // verify that the message was sent to the rx channel
        assert_eq!(rx.recv().unwrap(), chat_message);
    }

    #[test]
    fn test_handle_request() {
        let addr = "127.0.0.1:8849";
        let listener = TcpListener::bind(addr).unwrap();
        let (tx, rx) = mpsc::channel();

        // start handler for login request
        let listener_clone = listener.try_clone().unwrap();
        thread::spawn(move || {
            let s = listener_clone.accept().unwrap().0;
            let (_tx, _) = mpsc::channel();
            let clients = _make_clients(_tx);
            handle_request(s, rx, clients, 0).unwrap();
        });

        // send login request
        let mut login_stream = TcpStream::connect(addr).unwrap();
        let login_msg = LoginRequest::new_msg_string("Ben".into()).unwrap();
        write_line(&mut login_stream, login_msg).unwrap();

        // listen for successful status
        let good_status = Status::new_msg_string(false).unwrap();
        assert_eq!(read_line(&login_stream).unwrap(), good_status + "\n");

        // start handler for message request
        let listener_clone = listener.try_clone().unwrap();
        thread::spawn(move || {
            let s = listener_clone.accept().unwrap().0;
            let (_, _rx) = mpsc::channel();
            let clients = _make_clients(tx);
            handle_request(s, _rx, clients, 0).unwrap();
        });

        // send chat message
        let mut message_stream = TcpStream::connect(addr).unwrap();
        let chat_message = ChatMessage::new_msg_string("Hello".into(), "Ben".into()).unwrap();
        write_line(&mut message_stream, chat_message.clone()).unwrap();

        // listen for message
        assert_eq!(read_line(&login_stream).unwrap(), chat_message + "\n");
    }
}

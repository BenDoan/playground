use std::net::TcpStream;
use std::thread;
use std::env;
use std::process;
use std::sync::mpsc;
use std::error::Error;

use super::{ChatMessage, Message, LoginRequest, write_line, MsgType, read_line};

use cursive::Cursive;
use cursive::views::{Dialog, EditView, LinearLayout, TextView};
use cursive::view::ScrollStrategy;
use cursive::traits::*;

extern crate serde;
extern crate serde_json;

pub fn run_client() {
    let args: Vec<_> = env::args().collect();

    if args.len() != 4 {
        println!("Usage: chat-client (addr) (username)");
        process::exit(1);
    }

    let addr = args[2].to_string();
    let username = args[3].to_string();

    let (gui_sender, gui_receiver) = mpsc::channel();

    let addr_clone = addr.clone();
    let username_clone = username.clone();
    thread::spawn(move || if let Err(err) = start_message_listener(
        addr_clone,
        username_clone,
        &gui_sender,
    )
    {
        println!("Client error: {:?}", err);
    });

    start_ui(gui_receiver, addr, username);
}

fn start_ui(gui_receiver: mpsc::Receiver<ChatMessage>, addr: String, username: String) {
    let mut siv = Cursive::new();

    siv.set_fps(10);

    let text_view = TextView::new("")
        .scroll_strategy(ScrollStrategy::StickToBottom)
        .with_id("chat_msgs")
        .full_screen();

    let edit_view = EditView::new()
        .on_submit(move |s, msg_str| {
            send_chat_message(s, msg_str, addr.clone(), username.clone()).unwrap()
        })
        .with_id("edit");

    let layout = Dialog::new().title("Chat").content(
        LinearLayout::vertical()
            .child(text_view)
            .child(edit_view),
    );

    siv.add_layer(layout);

    loop {
        if let Ok(chat_message) = gui_receiver.try_recv() {
            if let Some(mut text_area) = siv.find_id::<TextView>("chat_msgs") {
                let msg_line = format!("\n{}: {}", chat_message.author, chat_message.text);
                text_area.append_content(msg_line.as_ref());
            }
        }

        siv.step();
    }
}

fn start_message_listener(
    addr: String,
    username: String,
    gui_sender: &mpsc::Sender<ChatMessage>,
) -> Result<(), Box<Error>> {
    let mut login_stream = TcpStream::connect(addr.clone())?;

    let login_request = LoginRequest::new_msg_string(username)?;
    write_line(&mut login_stream, login_request)?;

    loop {
        let buffer = read_line(&mut login_stream)?;
        let message: Message = serde_json::from_str(buffer.as_str())?;

        if message.msg_type == MsgType::ChatMessage {
            let chat_message: ChatMessage = serde_json::from_str(message.contents.as_ref())?;
            gui_sender.send(chat_message)?;
        }
    }
}

fn send_chat_message(
    s: &mut Cursive,
    msg_str: &str,
    addr: String,
    username: String,
) -> Result<(), Box<Error>> {
    let chat_message = ChatMessage::new_msg_string(msg_str.into(), username)?;
    let mut message_stream = TcpStream::connect(addr)?;
    write_line(&mut message_stream, chat_message)?;

    s.call_on_id("edit", |view: &mut EditView| { view.set_content(""); });

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Read;
    use std::net::TcpListener;
    use std::str::from_utf8;
    use std::thread;
    use super::*;

    #[test]
    fn test_listener_sends_messages_to_gui() {
        let addr = "127.0.0.1:8844";
        let listener = TcpListener::bind(addr).unwrap();
        let (tx, rx) = mpsc::channel();

        // start listener async
        thread::spawn(move || {
            start_message_listener(addr.into(), "Username".into(), &tx).unwrap()
        });

        // accept login
        let mut stream = listener.accept().unwrap().0;

        // send message to listener
        let msg = ChatMessage::new_msg_string("Hello".into(), "Author".into()).unwrap();
        write_line(&mut stream, msg.clone()).unwrap();

        // verify message received from listener
        assert_eq!(
            &rx.recv().unwrap(),
            &ChatMessage {
                author: "Author".into(),
                text: "Hello".into(),
            }
        );
    }

    #[test]
    fn test_send_chat_message() {
        let addr = "127.0.0.1:8845";
        let listener = TcpListener::bind(addr).unwrap();

        // send chat message async
        thread::spawn(move || {
            send_chat_message(
                &mut Cursive::new(),
                "hello".into(),
                addr.into(),
                "username".into(),
            ).unwrap();
        });

        let mut stream = listener.accept().unwrap().0;
        let msg = ChatMessage::new_msg_string("hello".into(), "username".into()).unwrap();

        // read chat message from stream
        let mut buf = [0; 84];
        stream.read(&mut buf).unwrap();

        // verify chat message contents
        assert_eq!(from_utf8(&buf).unwrap(), msg);
    }
}

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub msg_type: MsgType,
    pub contents: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum MsgType {
    LoginRequest,
    LoginResponse,
    ChatMessage,
    Status,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Msg {
    Status {error: bool},
    LoginRequest {username: String, password: String},
}

fn main() {
    let message = Message {
        msg_type: MsgType::Status,
        contents: "good".into()

    };

    println!("{:?}", message);
    println!("{:?}", serde_json::to_string(&message).unwrap());

    let msg = Msg::Status {
        error: false,
    };

    println!("{:?}", msg);
    println!("{:?}", serde_json::to_string(&msg).unwrap());

    let msg2: Msg = serde_json::from_str(serde_json::to_string(&msg).unwrap().as_ref()).unwrap();
    println!("{:?}", msg2);

    let msg3: Msg = Msg::Status {
        error: false
    };

    if let Msg::Status(error) = msg3 {
    }
}

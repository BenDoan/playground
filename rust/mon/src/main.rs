extern crate serde;
extern crate serde_json;
extern crate reqwest;
extern crate lettre;
extern crate lettre_email;

#[macro_use]
extern crate serde_derive;

use std::fs::File;
use std::error::Error;
use reqwest::StatusCode;
use std::{thread, time};
use lettre::{EmailTransport, SmtpTransport, smtp::response::Response};
use lettre::smtp::error::Error as MailError;
use lettre_email::EmailBuilder;

const CONFIG_LOCATION: &str = "./config.json";

#[derive(Serialize, Deserialize, Debug)]
struct Conf {
    addr: String,
    interval: u64,
}

fn ping(addr: String) -> Result<StatusCode, Box<Error>> {
    Ok(reqwest::get(&addr)?.status())
}

fn send_email(to: String, subject: String, body: String) -> Result<Response, MailError> {
    let email = EmailBuilder::new()
        .to((to.clone(), to.clone()))
        .from("mon")
        .subject(subject)
        .text(body)
        .build()
        .unwrap();

    let mut mailer = SmtpTransport::builder_unencrypted_localhost()
        .unwrap()
        .build();

    mailer.send(&email)
}

fn event_loop(conf: Conf) {
    loop {
        let code = ping(conf.addr.to_string());
        if code.is_err() || code.unwrap() != StatusCode::Ok {
            println!("Down");
        }
        thread::sleep(time::Duration::from_millis(conf.interval))
    }
}


fn main() -> Result<(), Box<Error>> {
    if let Ok(conf_file) = File::open(CONFIG_LOCATION) {
        let conf: Conf = serde_json::from_reader(conf_file)?;
        send_email("ben@bendoan.me".into(), "testing".into(), "Testing".into()).unwrap();
        event_loop(conf)
    } else {
        println!("Couldn't find config at '{}'", CONFIG_LOCATION);
    }
    Ok(())
}

use std::env;
use std::process;

extern crate chatlib;
use chatlib::client::run_client;
use chatlib::server::run_server;

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: chatbin (server (addr)) | (client (addr) (username))");
        process::exit(1);
    }

    match args[1].as_ref() {
        "server" => run_server(),
        "client" => run_client(),
        _ => process::exit(2),
    }
}

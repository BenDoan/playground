#![feature(phase)]
#[phase(plugin)]
extern crate regex_macros;
extern crate regex;
extern crate time;

use std::io::Command;
use std::str;
use std::string::String;
use std::io::Timer;
use std::num;
use std::io::{File, Open, Write};

fn main(){
    let p = Path::new("log.txt");
    let mut f = match File::open_mode(&p, Open, Write) {
        Ok(f) => f,
        Err(e) => fail!("file error: {}", e),
    };

    let mut timer = Timer::new().unwrap();
    let mut last_window = get_cur_window_name();
    let mut last_time = time::now();

    loop{
        let cur_window = get_cur_window_name();
        if last_window != cur_window {
            let time = time::now();
            let from = time.tm_min-last_time.tm_min;
            let to = num::abs(time.tm_sec-last_time.tm_sec);


            let outstr = format!("Changing to {}, {}m:{}s", cur_window, from, to);
            println!("{}", outstr);
            f.write_line(outstr.as_slice()).ok().unwrap();

            last_time = time;
        }

        last_window = get_cur_window_name();
        timer.sleep(1000);
    }
}

fn get_cur_window_name() -> String {
    let xprop_active_window = run_command("xprop", ["-root", "32x", "'\t$0''", "_NET_ACTIVE_WINDOW"]);
    let window_id = regex!(r"0x[0-9a-f]+").captures(xprop_active_window.as_slice()).unwrap().at(0);

    let net_wm_name = run_command("xprop", ["-id", window_id, "_NET_WM_NAME"]);
    let wm_name = regex!("= \"(.*)\"").captures(net_wm_name.as_slice()).unwrap().at(1);

    wm_name.replace("\\\"", "\"").to_string()
}

fn run_command(cmd: &str, args: &[&str]) -> String {
    let output = match Command::new(cmd).args(args).output(){
        Ok(output) => output,
        Err(e) => fail!("failed to execute process: {}", e),
    };

    str::from_utf8_lossy(output.output.as_slice()).to_string()
}

#[macro_use]
extern crate lalrpop_util;

use std::io;
use std::io::BufRead;

lalrpop_mod!(pub parser);

fn main() {
    let stdin = io::stdin();
    let program_string = stdin
        .lock()
        .lines()
        .filter_map(|l| l.ok())
        .collect::<String>();

    let result = parser::ProgramParser::new().parse(&program_string);
    println!("{:?}", result);
}

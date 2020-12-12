use std::env;
use std::fs;

const DEFAULT_FNAME: &str = "input.txt";

fn main() {
    let args: Vec<String> = env::args().collect();
    let default_fname = DEFAULT_FNAME.to_string();
    let fname = args.get(1).unwrap_or(&default_fname);
    let file = fs::read_to_string(fname).expect(&format!("Expected file named: {}", fname));
    let lines: Vec<&str> = file.trim().split("\n").collect();
    let mut seat_numbers: Vec<i32> = lines
        .iter()
        .map(|seat_ident| {
            let mut start = 0;
            let mut end = 127;
            let mut left = 0;
            let mut right = 7;
            for c in seat_ident.chars() {
                if c == 'F' {
                    end = end - ((end - start) / 2) - 1;
                } else if c == 'B' {
                    start = start + ((end - start) / 2) + 1;
                } else if c == 'L' {
                    right = right - ((right - left) / 2) - 1;
                } else if c == 'R' {
                    left = left + ((right - left) / 2) + 1;
                } else {
                    panic!("Unsupported seat ident")
                }
            }
            (start * 8) + left
        })
        .collect();

    println!("Part 1");
    let max_seat = seat_numbers.iter().max().unwrap();
    println!("max seat: {}", max_seat);

    println!("\nPart 2");
    seat_numbers.sort();
    for (i, seat_number) in seat_numbers.iter().enumerate() {
        let next_seat: i32 = *seat_numbers.get(i + 1).unwrap_or(&-1);
        if next_seat != seat_number + 1 {
            println!("My seat: {}", seat_number + 1);
            break;
        }
    }
}

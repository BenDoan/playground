use std::env;
use std::error::Error;
use std::fs;

fn main() {
    run().unwrap();
}

fn get_lines() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    let default_fname = "input.txt".into();
    let fname = args.get(1).unwrap_or(&default_fname);
    let file_string = fs::read_to_string(fname).expect(&format!("Expected file named: {}", fname));
    let lines: Vec<String> = file_string.trim().split("\n").map(|s| s.into()).collect();
    lines
}

fn run() -> Result<(), Box<dyn Error>> {
    let lines = get_lines();

    let changes: Vec<(i32, i32)> = lines
        .iter()
        .map(|line| {
            let split_line: Vec<&str> = line.split(" ").collect();
            let num = split_line[1].parse::<i32>().unwrap();
            match split_line[0] {
                "up" => (0, -num),
                "down" => (0, num),
                "forward" => (num, 0),
                _ => (0, 0),
            }
        })
        .collect();

    let horizontal: i32 = changes.iter().map(|change| change.0).sum();
    let depth: i32 = changes.iter().map(|change| change.1).sum();
    println!("Part 1: {}", horizontal * depth);

    let mut aim = 0;
    let changes: Vec<(i32, i32)> = lines
        .iter()
        .map(|line| {
            let split_line: Vec<&str> = line.split(" ").collect();
            let num = split_line[1].parse::<i32>().unwrap();
            match split_line[0] {
                "up" => {
                    aim -= num;
                    (0, 0)
                }
                "down" => {
                    aim += num;
                    (0, 0)
                }
                "forward" => (num, aim * num),
                _ => (0, 0),
            }
        })
        .collect();
    let horizontal: i32 = changes.iter().map(|change| change.0).sum();
    let depth: i32 = changes.iter().map(|change| change.1).sum();

    println!("Part 2: {}", horizontal * depth);

    Ok(())
}

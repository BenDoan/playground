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

fn find_num_increases(nums: &Vec<i32>) -> usize {
    nums.windows(2)
        .map(|pair| pair[1] - pair[0])
        .filter(|n| *n > 0)
        .collect::<Vec<i32>>()
        .len()
}

fn run() -> Result<(), Box<dyn Error>> {
    let lines = get_lines();
    let nums: Vec<i32> = lines.iter().map(|line| line.parse().unwrap()).collect();

    println!("Part 1: Count: {}", find_num_increases(&nums));

    let window_sums: Vec<i32> = nums.windows(3).map(|window| window.iter().sum()).collect();
    println!("Part 2: Count: {}", find_num_increases(&window_sums));

    Ok(())
}

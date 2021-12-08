use std::cmp::min;
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

fn part1(lines: &Vec<String>) -> Result<i32, Box<dyn Error>> {
    let mut crabs: Vec<i32> = lines
        .first()
        .unwrap()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();

    crabs.sort();

    let target = crabs.get(crabs.len() / 2).unwrap();
    Ok(crabs.iter().map(|crab| (target - crab).abs()).sum())
}

fn calc_exp_score(crabs: &Vec<i32>, target: i32) -> i32 {
    crabs
        .iter()
        .map(|crab| {
            let change = (target - crab).abs();
            (change * (change + 1)) / 2
        })
        .sum()
}

fn part2(lines: &Vec<String>) -> Result<i32, Box<dyn Error>> {
    let crabs: Vec<i32> = lines
        .first()
        .unwrap()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();
    let max = crabs.iter().max().unwrap();
    Ok((1..=*max).map(|n| calc_exp_score(&crabs, n)).min().unwrap())
}

fn run() -> Result<(), Box<dyn Error>> {
    let lines = get_lines();

    println!("Part 1: {}", part1(&lines)?);

    println!("Part 2: {}", part2(&lines)?);

    Ok(())
}

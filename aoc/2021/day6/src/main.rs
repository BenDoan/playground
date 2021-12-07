#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::cmp::min;
use std::env;
use std::error::Error;
use std::fs;

lazy_static! {
    static ref LINE_REGEX: Regex =
        Regex::new(r"(?P<x1>[0-9]+),(?P<y1>[0-9]+) -> (?P<x2>[0-9]+),(?P<y2>[0-9]+)").unwrap();
}

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

fn part1(lines: &Vec<String>) -> Result<usize, Box<dyn Error>> {
    let mut fishes: Vec<u32> = lines
        .get(0)
        .unwrap()
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect();

    for _ in 1..=80 {
        let mut new_fish_count = 0;
        for fish in fishes.iter_mut() {
            if *fish == 0 {
                *fish = 6;
                new_fish_count += 1;
            } else {
                *fish -= 1;
            }
        }

        for _ in 0..new_fish_count {
            fishes.push(8);
        }
    }
    Ok(fishes.len())
}

fn part2(lines: &Vec<String>) -> Result<u64, Box<dyn Error>> {
    let fishes: Vec<u32> = lines
        .get(0)
        .unwrap()
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect();
    let mut buckets = [0 as u64; 9];

    for fish in fishes {
        buckets[fish as usize] += 1;
    }

    for _ in 1..=256 {
        let new_fish = buckets[0];
        for i in 0..=7 {
            buckets[i] = buckets[i + 1];
        }
        buckets[6] += new_fish;
        buckets[8] = new_fish;
    }

    Ok(buckets.iter().sum())
}

fn run() -> Result<(), Box<dyn Error>> {
    let lines = get_lines();

    println!("Part 1: {}", part1(&lines)?);

    println!("Part 2: {}", part2(&lines)?);

    Ok(())
}

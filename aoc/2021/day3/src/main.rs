use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::fs;
use std::iter::FromIterator;

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

fn get_count<'a>(lines: impl Iterator<Item = &'a String>) -> [i32; 12] {
    let mut positions = [0; 12];
    for line in lines {
        for (i, c) in line.chars().enumerate() {
            positions[i] += if c == '1' { 1 } else { -1 }
        }
    }
    positions
}

fn get_rate(lines: &Vec<String>, get_max: bool) -> Result<isize, Box<dyn Error>> {
    let counts = get_count(lines.iter());
    Ok(isize::from_str_radix(
        &counts
            .iter()
            .map(|count| get_target(*count, get_max).to_string())
            .collect::<Vec<_>>()
            .join(""),
        2,
    )?)
}

fn get_rating(lines: &Vec<String>, get_max: bool) -> Result<isize, Box<dyn Error>> {
    let mut lines_set: HashSet<String> = HashSet::from_iter(lines.iter().cloned());
    for i in 0..12 {
        let count = get_count(lines_set.iter())[i];
        for line in &lines_set.clone() {
            let target = get_target(count, get_max);
            if line.chars().nth(i).unwrap() != target {
                lines_set.remove(line);
            }
        }
        if lines_set.len() == 1 {
            let result_string = &lines_set.iter().next().unwrap();
            return Ok(isize::from_str_radix(result_string, 2)?);
        }
    }
    return Ok(0);
}

fn get_target(count: i32, is_max: bool) -> char {
    if is_max {
        if count >= 0 {
            '1'
        } else {
            '0'
        }
    } else {
        if count >= 0 {
            '0'
        } else {
            '1'
        }
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let lines = get_lines();

    let gamma = get_rate(&lines, true)?;
    let epsilon = get_rate(&lines, false)?;

    println!("Part 1: {}*{}={}", gamma, epsilon, gamma * epsilon);

    let oxygen = get_rating(&lines, true)?;
    let c02 = get_rating(&lines, false)?;

    println!("Part 2: {}*{}={}", oxygen, c02, oxygen * c02);

    Ok(())
}

#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::cmp::min;
use std::env;
use std::error::Error;
use std::fs;

const SIZE: usize = 1000;

type Grid = [[usize; SIZE]; SIZE];

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

fn print_grid(grid: &Grid) {
    for row in grid {
        for cell in row {
            if *cell == 0 {
                print!(".");
            } else {
                print!("{}", cell);
            }
        }
        print!("\n");
    }
}

fn abs_diff(n1: usize, n2: usize) -> usize {
    let i1: i32 = n1 as i32;
    let i2: i32 = n2 as i32;
    (i1 - i2).abs() as usize
}

fn count_greater_equal_2(grid: &Grid) -> i32 {
    let mut count = 0;
    for row in grid {
        for cell in row {
            if *cell >= 2 {
                count += 1;
            }
        }
    }
    count
}

fn part1(lines: &Vec<String>) -> Result<i32, Box<dyn Error>> {
    let mut grid: Grid = [[0; SIZE]; SIZE];

    for line in lines {
        let caps = LINE_REGEX.captures(line).unwrap();

        let x1: usize = caps["x1"].parse()?;
        let y1: usize = caps["y1"].parse()?;
        let x2: usize = caps["x2"].parse()?;
        let y2: usize = caps["y2"].parse()?;

        if x1 == x2 {
            let m = min(y1, y2);
            for i in m..=m + abs_diff(y1, y2) {
                grid[i][x1] += 1;
            }
        }

        if y1 == y2 {
            let m = min(x1, x2);
            for i in m..=m + abs_diff(x1, x2) {
                grid[y2][i] += 1;
            }
        }
    }

    Ok(count_greater_equal_2(&grid))
}

fn part2(lines: &Vec<String>) -> Result<i32, Box<dyn Error>> {
    let mut grid: Grid = [[0; SIZE]; SIZE];

    for line in lines {
        let caps = LINE_REGEX.captures(line).unwrap();

        let x1: usize = caps["x1"].parse()?;
        let y1: usize = caps["y1"].parse()?;
        let x2: usize = caps["x2"].parse()?;
        let y2: usize = caps["y2"].parse()?;

        let x_delta: i32 = if x2 > x1 {
            1
        } else if x1 > x2 {
            -1
        } else {
            0
        };

        let y_delta: i32 = if y2 > y1 {
            1
        } else if y1 > y2 {
            -1
        } else {
            0
        };

        let (mut x, mut y) = (x1, y1);
        while x != x2 || y != y2 {
            grid[y][x] += 1;
            x = (x as i32 + x_delta) as usize;
            y = (y as i32 + y_delta) as usize;
        }
        grid[y2][x2] += 1;
    }

    Ok(count_greater_equal_2(&grid))
}

fn run() -> Result<(), Box<dyn Error>> {
    let lines = get_lines();

    println!("Part 1: {}", part1(&lines)?);

    println!("Part 2: {}", part2(&lines)?);

    Ok(())
}

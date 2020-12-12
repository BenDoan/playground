#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::collections::HashSet;
use std::env;
use std::fs;

const DEFAULT_FNAME: &str = "input.txt";

lazy_static! {
    static ref HGT_REGEX: Regex = Regex::new(r"(?P<height>\d+)(?P<unit>in|cm)").unwrap();
    static ref HCL_REGEX: Regex = Regex::new(r"#([0-9]|[a-f]){6}").unwrap();
    static ref VALID_EYE_COLORS: HashSet<String> =
        vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
            .into_iter()
            .map(|s| s.to_string())
            .collect();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let default_fname = DEFAULT_FNAME.to_string();
    let fname = args.get(1).unwrap_or(&default_fname);
    let file = fs::read_to_string(fname).expect(&format!("Expected file named: {}", fname));

    let passport_per_line: Vec<String> = file
        .replace("\n\n", "|")
        .replace("\n", " ")
        .trim()
        .split("|")
        .map(|s| s.to_string())
        .collect();
    let fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    println!("Part 1");
    let count = passport_per_line
        .iter()
        .filter(|passport| fields.iter().all(|field| passport.contains(field)))
        .count();
    println!("Count: {}", count);

    println!("\nPart 2");
    let count = passport_per_line
        .iter()
        .filter(|passport| fields.iter().all(|field| passport.contains(field)))
        .filter(|passport| {
            passport.split(" ").all(|field| {
                let split_field: Vec<&str> = field.split(":").collect();
                let field_name = split_field.get(0).expect("field name in pair");
                let field_val = split_field.get(1).expect("field val in pair");
                validate(field_name, field_val)
            })
        })
        .count();

    println!("Count: {}", count);
}

fn validate(field_name: &str, field_val: &str) -> bool {
    match field_name {
        "byr" => {
            let year = field_val.parse::<i32>();
            if let Ok(year) = year {
                year >= 1920 && year <= 2002
            } else {
                false
            }
        }
        "iyr" => {
            let year = field_val.parse::<i32>();
            if let Ok(year) = year {
                year >= 2010 && year <= 2020
            } else {
                false
            }
        }
        "eyr" => {
            let year = field_val.parse::<i32>();
            if let Ok(year) = year {
                year >= 2020 && year <= 2030
            } else {
                false
            }
        }
        "hgt" => {
            let caps = HGT_REGEX.captures(field_val);
            if let Some(caps) = caps {
                let height = caps["height"].parse::<u32>();
                let unit = &caps["unit"];
                if let Ok(height) = height {
                    if unit == "in" {
                        height >= 59 && height <= 76
                    } else if unit == "cm" {
                        height >= 150 && height <= 193
                    } else {
                        panic!("impossible height value")
                    }
                } else {
                    false
                }
            } else {
                false
            }
        }
        "hcl" => HCL_REGEX.is_match(field_val),
        "ecl" => VALID_EYE_COLORS.contains(field_val),
        "pid" => field_val.len() == 9,
        "cid" => true,
        _ => false,
    }
}

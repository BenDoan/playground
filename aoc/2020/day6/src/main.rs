use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;

const DEFAULT_FNAME: &str = "input.txt";

fn main() {
    let args: Vec<String> = env::args().collect();
    let default_fname = DEFAULT_FNAME.to_string();
    let fname = args.get(1).unwrap_or(&default_fname);
    let file_string = fs::read_to_string(fname).expect(&format!("Expected file named: {}", fname));
    let groups: Vec<String> = file_string
        .replace("\n\n", "|")
        .replace("\n", "")
        .split("|")
        .map(|s| s.to_string())
        .collect();

    println!("Part 1");
    let answer_sum: usize = groups
        .iter()
        .map(|group| {
            let answers: HashSet<char> = group.chars().into_iter().collect();
            answers.len()
        })
        .sum();
    println!("Answer sum: {}", answer_sum);

    println!("\nPart 2");
    let groups: Vec<String> = file_string
        .trim()
        .replace("\n\n", "|")
        .replace("\n", "-")
        .split("|")
        .map(|s| s.to_string())
        .collect();

    let answers_sum: usize = groups
        .iter()
        .map(|group| {
            let members = group.split("-").collect::<Vec<&str>>();
            let members_len = members.len();

            let mut group_answers: HashMap<char, usize> = HashMap::new();
            for member in members {
                for c in member.chars() {
                    group_answers.insert(c, group_answers.get(&c).unwrap_or(&0).clone() + 1);
                }
            }

            let num_answered_by_all = group_answers
                .values()
                .filter(|&&count| count == members_len)
                .count();
            num_answered_by_all
        })
        .sum();

    println!("Sum: {}", answers_sum)
}

use regex::Regex;
use std::fs;

fn main() {
    // format: "1-3 a: abcde"
    let re =
        Regex::new(r"(?P<range_start>\d+)-(?P<range_end>\d+) (?P<letter>\w): (?P<password>.*)")
            .unwrap();

    println!("Part 1");
    let file = fs::read_to_string("input.txt").expect("File with the name input.txt");
    let lines: Vec<&str> = file.trim().split("\n").collect();
    let num_valid_passwords = lines
        .iter()
        .filter(|line| {
            let caps = re.captures(line).unwrap();

            let range_start = &caps["range_start"].parse::<usize>().unwrap();
            let range_end = &caps["range_end"].parse::<usize>().unwrap();
            let letter = &caps["letter"];
            let password = &caps["password"];

            let num_matches = password.matches(letter).count();
            num_matches >= *range_start && num_matches <= *range_end
        })
        .count();

    println!("Num valid: {:?}", num_valid_passwords);

    println!("\nPart 2");
    let num_valid_passwords = lines
        .iter()
        .filter(|line| {
            let caps = re.captures(line).unwrap();

            let range_start = caps["range_start"].parse::<usize>().unwrap();
            let range_end = caps["range_end"].parse::<usize>().unwrap();
            let letter = &caps["letter"];
            let password = &caps["password"];

            let pos1_matches = password.get(range_start - 1..range_start).unwrap() == letter;
            let pos2_matches = password.get(range_end - 1..range_end).unwrap() == letter;
            (pos1_matches || pos2_matches) && !(pos1_matches && pos2_matches)
        })
        .count();
    println!("Num valid: {:?}", num_valid_passwords);
}

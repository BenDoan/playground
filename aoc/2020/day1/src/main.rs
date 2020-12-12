use std::collections::HashSet;
use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt").expect("File with the name input.txt");
    let nums: Vec<i32> = file
        .split("\n")
        .map(|num_str| num_str.parse::<i32>())
        .filter_map(|x| x.ok())
        .collect();
    let num_set: HashSet<i32> = nums.into_iter().collect();

    println!("Part 1");
    for num in num_set.iter() {
        let desired_num = 2020 - num;
        if num_set.contains(&desired_num) && num != &desired_num {
            println!(
                "Found num: {} * {} = {}",
                num,
                desired_num,
                num * desired_num
            );
        }
    }

    println!("\nPart 2");
    for num in num_set.iter() {
        let intermediate_desired_num = 2020 - num;
        for num2 in num_set.iter() {
            let desired_num = intermediate_desired_num - num2;
            if num_set.contains(&desired_num) {
                println!(
                    "Found num: {} * {} * {} = {}",
                    num,
                    num2,
                    desired_num,
                    num * num2 * desired_num
                );
            }
        }
    }
}

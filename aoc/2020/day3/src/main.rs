use std::env;
use std::fs;

const DEFAULT_FNAME: &str = "input.txt";

fn main() {
    let args: Vec<String> = env::args().collect();
    let default_fname = DEFAULT_FNAME.to_string();
    let fname = args.get(1).unwrap_or(&default_fname);
    let file = fs::read_to_string(fname).expect(&format!("Expected file named: {}", fname));

    let lines: Vec<&str> = file.trim().split("\n").collect();
    let line_len = lines.get(0).unwrap().len();

    println!("Part 1");
    let num_trees: usize = (1..lines.len())
        .filter(|&level| {
            let target_width = (level * 3) % (line_len);
            let target = lines
                .get(level)
                .expect("getting level")
                .get(target_width..target_width + 1)
                .expect("getting char");
            target == "#"
        })
        .count();
    println!("Num trees: {}", num_trees);

    println!("\nPart 2");
    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let nums_trees: Vec<usize> = slopes
        .iter()
        .map(|(run, rise)| {
            let num_trees: usize = (*rise..lines.len())
                .step_by(*rise)
                .enumerate()
                .filter(|(i, level)| {
                    let target_width = ((i + 1) * run) % (line_len);
                    let target = lines
                        .get(*level)
                        .expect("getting level")
                        .get(target_width..target_width + 1)
                        .expect("getting char");
                    target == "#"
                })
                .count();
            num_trees
        })
        .collect();

    let product: usize = nums_trees.iter().fold(1, |a, b| a * b);
    println!("Product: {}", product);
}

use std::env;
use std::error::Error;
use std::fs;

type Board<'a> = Vec<Vec<&'a str>>;
type Boards<'a> = Vec<Board<'a>>;

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

fn print_board(board: &Board) {
    for row in board {
        for col in row {
            print!("{: >2} ", col);
        }
        print!("\n");
    }
    print!("\n");
}

fn print_boards(boards: &Boards) {
    for board in boards {
        print_board(&board);
    }
    println!("=============\n")
}

fn build_boards(lines: &Vec<String>) -> Boards {
    let mut boards: Boards = vec![];
    let mut curr_board: Board = vec![];
    for line in &lines[2..lines.len()] {
        if line == "" {
            boards.push(curr_board);
            curr_board = vec![];
            continue;
        }
        let nums_in_row: Vec<&str> = line
            .split(" ")
            .map(|num| num.trim())
            .filter(|num| num != &"")
            .collect();
        curr_board.push(nums_in_row);
    }
    boards.push(curr_board);
    boards
}

fn mark_boards(drawing: &str, boards: &mut Boards) {
    for board in boards {
        for row in board {
            for cell in row {
                if *cell == drawing {
                    *cell = "*";
                }
            }
        }
    }
}

fn has_win_by_row(board: &Board) -> bool {
    board.iter().any(|row| row.iter().all(|cell| *cell == "*"))
}

fn has_win_by_col(board: &Board) -> bool {
    for i in 0..board.len() {
        if board
            .iter()
            .map(|row| *row.get(i).unwrap())
            .all(|cell| cell == "*")
        {
            return true;
        }
    }
    false
}

fn has_win(board: &Board) -> bool {
    has_win_by_row(board) || has_win_by_col(board)
}

fn calc_score(drawing: &str, board: &Board) -> i32 {
    let mut sum: i32 = 0;
    for row in board {
        for cell in row {
            if *cell != "*" {
                sum += cell.parse::<i32>().unwrap();
            }
        }
    }
    sum * drawing.parse::<i32>().unwrap()
}

fn part1(lines: &Vec<String>, drawings: &Vec<&str>) -> i32 {
    let mut boards = build_boards(lines);

    for drawing in drawings {
        mark_boards(drawing, &mut boards);

        for board in &boards {
            if has_win(board) {
                return calc_score(drawing, board);
            }
        }
    }
    0
}

fn part2(lines: &Vec<String>, drawings: &Vec<&str>) -> i32 {
    let mut boards = build_boards(lines);

    let mut winning_score = 0;
    for drawing in drawings {
        mark_boards(drawing, &mut boards);

        let mut to_remove: Vec<usize> = vec![];
        for (i, board) in boards.iter().enumerate() {
            if has_win(board) {
                winning_score = calc_score(drawing, board);
                to_remove.push(i);
            }
        }

        to_remove.sort();

        for i in to_remove.iter().rev() {
            boards.swap_remove(*i);
        }

        if boards.len() == 0 {
            return winning_score;
        }
    }
    0
}

fn run() -> Result<(), Box<dyn Error>> {
    let lines = get_lines();

    let drawings: Vec<&str> = lines.first().unwrap().split(",").collect();

    println!("Part 1: {}", part1(&lines, &drawings));

    println!("Part 2: {}", part2(&lines, &drawings));

    Ok(())
}

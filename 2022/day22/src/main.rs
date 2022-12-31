use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec::Vec;

// copy/paste from Rust By Example
// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug, PartialEq)]
enum Direction {
    Right,
    Bottom,
    Left,
    Up,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let mut last_line = false;
    let mut instructions = String::new();

    let mut rows: Vec<(usize, usize, Vec<u8>)> = Vec::new();
    rows.push((0, 0, Vec::new()));

    if let Ok(lines) = read_lines(&file_path) {
        let lines = lines.map(|l| l.unwrap());
        for line in lines {
            if last_line {
                instructions = line.clone()
            }
            if line.len() == 0 {
                last_line = true;
                continue;
            }
            let len1 = line.len();
            let len2 = line.trim_start().len();
//            println!("{} {}", len1, len2);

            let row = Vec::from_iter(line.trim_start().bytes());
            let start = len1 - len2 + 1;
            let end = len1 + 1;
            rows.push((start, end, row));
        }
    }

//    println!("{:?}", rows);
//    println!("{}", instructions);

    rows.push((0, 0, Vec::new()));

    let nrows = rows.len();

    let mut direction = Direction::Right;
    let mut row: usize = 1;
    let mut col: usize = rows[1].0;

    for instr in instructions.split_inclusive(&['R', 'L']) {
        println!("{}", instr);
        let steps: usize = instr.trim_end_matches(&['R', 'L']).parse().unwrap();
        println!("{}", steps);
        for _ in 0..steps {
            let (mut next_col, mut next_row) = match direction {
                Direction::Right => (col + 1, row),
                Direction::Bottom => (col, row + 1),
                Direction::Left => (col - 1, row),
                Direction::Up => (col, row - 1),
            };
            if direction == Direction::Up {
                let (start, end, _) = &rows[next_row];
                if *start > col || *end <= col {
                    for nrow in (0..nrows).rev() {
                        let (start, end, _) = &rows[nrow];
                        if col >= *start && col < *end {
                            next_row = nrow;
                            break;
                        }
                    }
                }
            }
            if direction == Direction::Bottom {
                let (start, end, _) = &rows[next_row];
                if *start > col || *end <= col {
                    for nrow in 0..nrows {
                        let (start, end, _) = &rows[nrow];
                        if col >= *start && col < *end {
                            next_row = nrow;
                            break;
                        }
                    }
                }
            }
            let (start, end, row_vec) = &rows[next_row];
            if direction == Direction::Left && next_col < *start {
                next_col = *end - 1;
            } else if direction == Direction::Right && next_col >= *end {
                next_col = *start;
            }
            println!("moving {:?} to row {} col {}", direction, next_row, next_col);
            if row_vec[next_col - *start] == b'.' {
                row = next_row;
                col = next_col;
            } else {
                println!("found wall, stopping");
                break;
            }
        }
        if instr.ends_with('L') {
            direction = match direction {
                Direction::Right => Direction::Up,
                Direction::Up => Direction::Left,
                Direction::Left => Direction::Bottom,
                Direction::Bottom => Direction::Right,
            };
        } else if instr.ends_with('R') {
            direction = match direction {
                Direction::Right => Direction::Bottom,
                Direction::Bottom => Direction::Left,
                Direction::Left => Direction::Up,
                Direction::Up => Direction::Right,
            };
        }
        println!("new direction: {:?}", direction);
    }
    let mut password = 1000 * row + 4 * col;
    password += match direction {
        Direction::Right => 0,
        Direction::Bottom => 1,
        Direction::Left => 2,
        Direction::Up => 3,
    };
    println!("password is: {}", password);
}

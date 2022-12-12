use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec::Vec;
use std::collections::VecDeque;

// copy/paste from Rust By Example
// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

struct Place {
    height: u8,
    steps: u32,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let mut rows: Vec<Vec<Place>> = Vec::new();
    let mut todo = VecDeque::<((i32, i32), u8, u32)>::new();
    let mut start: (i32, i32) = (0, 0);
    let mut end: (i32, i32) = (0, 0);

    if let Ok(lines) = read_lines(&file_path) {
        // Consumes the iterator, returns an (Optional) String
        let mut nrow = 0;
        for line in lines {
            if let Ok(line) = line {
                rows.push(Vec::from_iter(line.bytes().enumerate().map(|(ncol, x)| {
                    Place {
                        height: match x {
                            b'S' => {
                                start = (nrow, ncol.try_into().unwrap());
                                0
                            }
                            b'E' => {
                                end = (nrow, ncol.try_into().unwrap());
                                b'z' - b'a'
                            }
                            _ => x - b'a',
                        },
                        steps: u32::MAX,
                    }
                })));
            }
            nrow += 1;
        }
    }

    todo.push_back((start, 0, 0));

    let nrows: i32 = rows.len().try_into().unwrap();
    let ncols: i32 = rows[0].len().try_into().unwrap();
    while let Some(((row, col), height, steps)) = todo.pop_front() {
        let mut place = &mut rows[usize::try_from(row).unwrap()][usize::try_from(col).unwrap()];
        if place.steps <= steps {
            continue;
        }
        if place.height > height + 1 {
            continue
        }
        place.steps = steps;
        for (row, col) in [(row-1, col), (row+1, col), (row, col-1), (row, col+1)] {
            if row < 0 || col < 0 || row >= nrows || col >= ncols {
                continue;
            }
            todo.push_back(((row, col), place.height, steps + 1));
        }
    }

    let end_place = &rows[usize::try_from(end.0).unwrap()][usize::try_from(end.1).unwrap()];
    println!("Reached end place in {} steps", end_place.steps);

    // Part Two
    for row in rows.iter_mut() {
        for place in row.iter_mut() {
            place.steps = u32::MAX;
        }
    }

    todo.push_back((end, b'z' - b'a', 0));

    while let Some(((row, col), height, steps)) = todo.pop_front() {
        let mut place = &mut rows[usize::try_from(row).unwrap()][usize::try_from(col).unwrap()];
        if place.steps <= steps {
            continue;
        }
        if place.height + 1 < height {
            continue
        }
        place.steps = steps;

        if place.height == 0 {
            println!("Reached an 'a' at ({} {}) from 'E' in {} steps", row, col, steps);
            break;
        }

        for (row, col) in [(row-1, col), (row+1, col), (row, col-1), (row, col+1)] {
            if row < 0 || col < 0 || row >= nrows || col >= ncols {
                continue;
            }
            todo.push_back(((row, col), place.height, steps + 1));
        }
    }
}

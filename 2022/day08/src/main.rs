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

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let mut rows: Vec<Vec<(u8, bool)>> = Vec::new();

    if let Ok(lines) = read_lines(&file_path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line) = line {
                rows.push(Vec::from_iter(line.bytes().map(|x| (x - b'0' + 1, false))));
            }
        }
    }

    for _ in 0..2 {
        for row in rows.iter_mut() {
            let mut highest: u8 = 0;
            for (val, vis) in row.iter_mut() {
                if *val > highest {
                    *vis = true;
                    highest = *val;
                }
            }
            row.reverse();
        }
    }

    let ncols = rows[0].len();
    for _ in 0..2 {
        for ncol in 0..ncols {
            let mut highest: u8 = 0;
            for row in rows.iter_mut() {
                let (val, vis) = &mut row[ncol];
                if *val > highest {
                    *vis = true;
                    highest = *val;
                }
            }
        }
        rows.reverse();
    }

    let mut nvis = 0;
    for row in rows.iter() {
        for (_, vis) in row.iter() {
            if *vis {
                nvis += 1;
            }
        }
    }
    //println!("{rows:?}");
    let nrows = rows.len();

    let mut most_scenic = 0;
    for nrow in 1..nrows - 1 {
        for ncol in 1..ncols - 1 {
            let mut score = 1;
            let h = rows[nrow][ncol].0;

            // look left
            let mut vis = 0;
            for i in (0..ncol).rev() {
                vis += 1;
                if rows[nrow][i].0 >= h {
                    break;
                }
            }
            score *= vis;

            // loook right
            vis = 0;
            for i in ncol+1..ncols {
                vis += 1;
                if rows[nrow][i].0 >= h {
                    break;
                }
            }
            score *= vis;

            // look up
            vis = 0;
            for i in (0..nrow).rev() {
                vis += 1;
                if rows[i][ncol].0 >= h {
                    break;
                }
            }
            score *= vis;

            // look down
            vis = 0;
            for i in nrow+1..nrows {
                vis += 1;
                if rows[i][ncol].0 >= h {
                    break;
                }
            }
            score *= vis;

            if score > most_scenic {
                most_scenic = score;
            }
        }
    }

    println!("visible trees: {nvis}");
    println!("most scenic: {most_scenic}");
}

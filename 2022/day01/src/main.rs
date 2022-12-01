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

    let mut total: u64 = 0;
    let mut totals: Vec<u64> = Vec::new();

    if let Ok(lines) = read_lines(&file_path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line) = line {
                match line.trim().parse::<u64>() {
                    Ok(num) => {
                        total += num;
                    },
                    Err(_) => {
                        totals.push(total);
                        total = 0;
                    },
                }
            }
        }
    }
    // if I figured out how to chain lines with extra empty one, this
    // would not be necessary
    if total > 0 {
        totals.push(total);
    }
    totals.sort();
    total = totals.pop().expect("no elf at all?");
    println!("the elf with most calories has: {total}");
    for _ in 1..3 {
        total += totals.pop().expect("less than 3 elves");
    }
    println!("the 3 elves with most calories have total: {total}");
}

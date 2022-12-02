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
    let mut score = 0;
    let mut score2 = 0;

    if let Ok(lines) = read_lines(&file_path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line) = line {
                let mut chars = line.chars();

                let elf_move = chars.next().expect("elf move missing");
                chars.next();
                let my_move = chars.next().expect("my move missing");

                let elf_move = match elf_move {
                    'A' => 1,
                    'B' => 2,
                    'C' => 3,
                    _ => panic!("unknown elf move {elf_move}"),
                };

                let (my_move, my_score2) = match my_move {
                    'X' => (1, 0), // lose
                    'Y' => (2, 3), // draw
                    'Z' => (3, 6), // win
                    _ => panic!("unknown elf move {elf_move}"),
                };

                score += my_move;
                if elf_move == my_move {
                    score += 3;
                } else if my_move - 1 == elf_move % 3 {
                    score += 6;
                }

                // strategy 2
                score2 += my_score2;
                let mut my_move = (elf_move + (my_move - 2)) % 3;
                // uglyyyyy
                if my_move == 0 {
                    my_move = 3;
                }
                score2 += my_move;
            }
        }
    }
    println!("my score in the first strategy is: {score}");
    println!("my score in the second strategy is: {score2}");
}

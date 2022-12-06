use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec::Vec;
use std::collections::HashSet;
use std::collections::hash_map::RandomState;

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
    let msglen: usize = args[2].parse().expect("msglen invalid");

    if let Ok(lines) = read_lines(&file_path) {
        for line in lines {
            if let Ok(line) = line {
                let len = line.len();
                for i in 0..len-msglen {
                    let chunk = &line[i..i+msglen];
                    let set: HashSet<char, RandomState> = HashSet::from_iter(chunk.chars());
                    if set.len() == msglen {
                        println!("first marker after character: {}", i+msglen);
                        break;
                    }
                }
            }
        }
    }
}

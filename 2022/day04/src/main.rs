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
    let mut contained = 0;
    let mut overlaps = 0;

    if let Ok(lines) = read_lines(&file_path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line) = line {
                let elves: Vec<&str> = line.split(",").collect();
                let elf1: Vec<&str> = elves[0].split("-").collect();
                let elf2: Vec<&str> = elves[1].split("-").collect();
                let mut elf1_num: [u64; 2] = [0; 2];
                let mut elf2_num: [u64; 2] = [0; 2];
                for i in 0..2 {
                    elf1_num[i] = elf1[i].parse().expect("Not a number");
                    elf2_num[i] = elf2[i].parse().expect("Not a number");
                }
                if (elf1_num[0] <= elf2_num[0] && elf1_num[1] >= elf2_num[1]) ||
                   (elf2_num[0] <= elf1_num[0] && elf2_num[1] >= elf1_num[1]) {
                    //println!("{}-{},{}-{}", elf1_num[0], elf1_num[1], elf2_num[0], elf2_num[1]);
                    contained += 1;
                }
                if !(elf1_num[1] < elf2_num[0] || elf1_num[0] > elf2_num[1]) {
                    println!("{}-{},{}-{}", elf1_num[0], elf1_num[1], elf2_num[0], elf2_num[1]);
                    overlaps += 1;
                }
            }
        }
        println!("contained: {contained}");
        println!("overlaps: {overlaps}");
    }
}

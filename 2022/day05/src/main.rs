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
    let mut init = false;
    let mut num_stacks: usize = 0;
    let mut stacks = Vec::<Vec::<char>>::new();
    let mut empty_line = false;
    let mut instructions = false;

    if let Ok(lines) = read_lines(&file_path) {
        // Consumes the iterator, returns an (Optional) String
        'out: for line in lines {
            if let Ok(line) = line {
                if empty_line {
                    empty_line = false;
                    instructions = true;
                    for i in 0..num_stacks {
                        stacks[i].reverse();
                    }
                    continue;
                }
                if !init {
                    num_stacks = line.len() / 4 + 1;
                    for _ in 0..num_stacks {
                        stacks.push(Vec::new());
                    }
                    init = true;
                }
                if !instructions {
                    for i in 0..num_stacks {
                        let start = i*4;
                        let end = (i+1)*4 - 1;
                        let chunk = &line[start..end];
                        if chunk.eq(" 1 ") {
                            empty_line = true;
                            continue 'out;
                        }
                        let mut chunk = chunk.chars();
                        let first = chunk.next().expect("bad chunk");
                        if first == '[' {
                            let crt: char = chunk.next().expect("bad chunk");
                            stacks[i].push(crt);
                        }
                    }
                    continue
                }
                let mut instr = line.split(" ");
                assert_eq!(instr.next().expect("missing 'move'"), "move");
                let num: usize = instr.next().expect("missing 'num'")
                                       .parse().expect("'num' not number");
                assert_eq!(instr.next().expect("missing 'from'"), "from");
                let from: usize = instr.next().expect("missing 'from'")
                                       .parse().expect("'from' not number");
                assert_eq!(instr.next().expect("missing 'to'"), "to");
                let to: usize = instr.next().expect("missing 'to'")
                                       .parse().expect("'to' not number");
                let mut crane = Vec::<char>::new();
                for _ in 0..num {
                    let crt = stacks[from - 1].pop().expect("stack empty");
                    crane.push(crt);
                }
                crane.reverse(); // comment out to downgrade from CrateMover 9001 to 9000
                stacks[to - 1].append(&mut crane);
            }
        }
    }
    let mut result = String::new();
    for i in 0..num_stacks {
        let crt = stacks[i].pop().expect("stack empty");
        result.push(crt);
    }
    println!("Result: {result}");
}

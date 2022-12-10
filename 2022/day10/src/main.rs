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

struct State {
    clock: i32,
    x:  i32,
    next_check: Option<i32>,
    sum_signals: i32,
    line: String,
}

impl State {
    fn new() -> Self {
        Self {
            clock: 0,
            x : 1,
            next_check: None,
            sum_signals: 0,
            line: String::new(),
        }
    }

    fn inc_clock(&mut self, check_iter: &mut dyn Iterator<Item = i32>) {
        if self.next_check.is_none() {
            self.next_check = check_iter.next();
        }

        let xpos: i32 = self.clock % 40;

        if self.x-1 <= xpos && self.x+1 >= xpos {
            self.line.push('#');
        } else {
            self.line.push('.');
        }

        self.clock += 1;

        match self.next_check {
            Some(check) => {
                if self.clock == check {
                    self.sum_signals += self.x * self.clock;
                    self.next_check = None;
                }
            }
            None => ()
        }

        if self.clock % 40 == 0 {
            println!("{}", self.line);
            self.line.clear();
        }

    }

    fn change_x(&mut self, diff: i32) {
        self.x += diff;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let mut state = State::new();
    let mut check_iter = (20..=220).step_by(40);

    if let Ok(lines) = read_lines(&file_path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line) = line {
                let mut instr = line.split(" ");

                let ins = instr.next().expect("missing direction");

                if ins.eq("noop") {
                    state.inc_clock(&mut check_iter);
                } else if ins.eq("addx") {
                    let diff: i32 = instr.next().expect("missing number after addx")
                                                .parse().expect("can't parse number after addx");
                    state.inc_clock(&mut check_iter);
                    state.inc_clock(&mut check_iter);
                    state.change_x(diff);
                } else {
                    panic!("unknown instruction: {line}");
                }
            }
        }
        println!("Sum of signal strengths is {}", state.sum_signals);
    }
}

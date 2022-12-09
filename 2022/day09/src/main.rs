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

#[derive(Clone, Debug, Copy)]
#[derive(Eq, Hash, PartialEq)]
struct Pos(i32, i32);

impl Pos {
    fn move_in_direction(&mut self, direction: char) {
        match direction {
            'R' => self.0 += 1,
            'L' => self.0 -= 1,
            'U' => self.1 += 1,
            'D' => self.1 -= 1,
            _ => panic!("unknown move direction {direction}"),
        }
    }
    fn move_towards(&mut self, other: &Pos) {
        if self.0 < other.0 - 1 {
            self.0 = other.0 - 1;
            self.1 += (other.1 - self.1).signum();
        } else if self.0 > other.0 + 1 {
            self.0 = other.0 + 1;
            self.1 += (other.1 - self.1).signum();
        } else if self.1 < other.1 - 1 {
            self.1 = other.1 - 1;
            self.0 += (other.0 - self.0).signum();
        } else if self.1 > other.1 + 1 {
            self.1 = other.1 + 1;
            self.0 += (other.0 - self.0).signum();
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let mut knots = [Pos(0, 0); 10];
    let mut visits: HashSet<Pos, RandomState> = HashSet::new();

    visits.insert(knots[knots.len()-1]);
    if let Ok(lines) = read_lines(&file_path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line) = line {
                let mut instr = line.split(" ");

                let direction = instr.next().expect("missing direction").chars().next().expect("missing direction");

                let moves: u32 = instr.next().expect("missing number of moves")
                                       .parse().expect("number of moves not number");

                for _ in 0..moves {
                    knots[0].move_in_direction(direction);
                    for knot in 1..knots.len() {
                        // this is rather lame to avoid mutable vs immutable borrow
                        let tmp = knots[knot-1].clone();
                        knots[knot].move_towards(&tmp);
                    }
                    visits.insert(knots[knots.len()-1]);
                }
            }
        }
        println!("Visited fields: {}", visits.len());
    }
}

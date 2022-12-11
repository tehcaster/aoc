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

#[derive(Debug)]
enum OpOper {
    Plus,
    Multiple,
}

#[derive(Debug)]
enum OpSecond {
    Old,
    Num(i64),
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<i64>,
    op_oper: OpOper,
    op_second: OpSecond,
    test_div: i64,
    throw_true: usize,
    throw_false: usize,
}

impl Monkey {
    fn inspect(&mut self) -> Option<(i64, usize)> {
        if let Some(item) = self.items.pop_front() {
            let mut worry: i64 = item;
            let second: i64 = match self.op_second {
                OpSecond::Old => item,
                OpSecond::Num(w) => w,
            };
            worry = match self.op_oper {
                OpOper::Plus => worry + second,
                OpOper::Multiple => worry * second,
            };
            // uncomment for part1
            // worry = worry / 3;
            let divisible: bool = worry % self.test_div == 0;
            let target_monkey: usize = match divisible {
                true => self.throw_true,
                false => self.throw_false,
            };
            Some((worry, target_monkey))
        } else {
            None
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let rounds: u32 = args[2].parse().expect("rounds");
    let mut monkeys: Vec<Monkey> = Vec::new();
    let mut div_rnd = 1;

    if let Ok(lines) = read_lines(&file_path) {
        let mut lines = lines.map(|l| l.unwrap());
        loop {
            let monkey = lines.next();
            if monkey.is_none() {
                break;
            }

            let line = lines.next().expect("starting items");
            let items = line.strip_prefix("  Starting items:").expect("starting items");
            let items = items.split(",");
            let mut item_vec = VecDeque::<i64>::new();
            for item in items {
                let val: i64 = item.trim().parse().expect("item worry level");
                item_vec.push_back(val);
            }

            let line = lines.next().expect("operation");
            let op = line.strip_prefix("  Operation: new = old ").expect("operation");
            let mut op = op.split(" ");
            let oper = op.next().expect("operator").chars().next().expect("operator char");
            let oper: OpOper = match oper {
                '+' => OpOper::Plus,
                '*'=> OpOper::Multiple,
                _ => todo!("operator {oper}"),
            };

            let op_second = op.next().expect("operation second");
            let op_second: OpSecond = {
                if op_second.eq("old") {
                    OpSecond::Old
                } else {
                    OpSecond::Num(op_second.parse().expect("operation second"))
                }
            };

            let line = lines.next().expect("test");
            let test = line.strip_prefix("  Test: divisible by ").expect("test");
            let test_div: i64 = test.trim().parse().expect("divisible by");
            div_rnd *= test_div;

            let line = lines.next().expect("if true");
            let throw = line.strip_prefix("    If true: throw to monkey ").expect("if true");
            let throw1: usize = throw.trim().parse().expect("throw if true");

            let line = lines.next().expect("if false");
            let throw = line.strip_prefix("    If false: throw to monkey ").expect("if false");
            let throw2: usize = throw.trim().parse().expect("throw if false");

            let _ = lines.next();

            let monkey = Monkey {
                items: item_vec,
                op_oper: oper,
                op_second: op_second,
                test_div: test_div,
                throw_true: throw1,
                throw_false: throw2,
            };

            monkeys.push(monkey);
        }

        let num_monkeys = monkeys.len();
        let mut inspections = vec![0u64; num_monkeys];
        for _ in 0..rounds {
            for turn in 0..num_monkeys {
                loop {
                    let monkey = &mut monkeys[turn];
                    match monkey.inspect() {
                        None => break,
                        Some((item, throw_target)) => {
                            inspections[turn] += 1;
                            let target = &mut monkeys[throw_target];
                            target.items.push_back(item % div_rnd);
                        }
                    }
                }
            }
        }
        inspections.sort_by(|a, b| b.cmp(a));
        println!("monkey business is: {}", inspections[0] * inspections[1]);
    }
}

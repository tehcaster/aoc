use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec::Vec;
use std::cmp::{min, max};
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use regex::Regex;

// copy/paste from Rust By Example
// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

struct MonkeyOp {
    param1: String,
    param2: String,
    op: char,
}

#[derive(PartialEq, Debug)]
enum HumnDerived {
    No,
    Left,
    Right,
    Num,
}

struct Monkey {
    num: Option<i64>,
    op: Option<MonkeyOp>,
    humn_derived: HumnDerived,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    // dbpl: 5
    let re_num = Regex::new(r"([a-z]+): (\d+)").unwrap();
    // ptdq: humn - dvpt
    let re_op = Regex::new(r"([a-z]+): ([a-z]+) (.) ([a-z]+)").unwrap();

    if let Ok(lines) = read_lines(&file_path) {
        let mut monkeys: HashMap<String, Monkey> = HashMap::new();
        let mut queue: VecDeque<String> = VecDeque::new();
        let mut queued: HashSet<String> = HashSet::new();

        let lines = lines.map(|l| l.unwrap());
        for line in lines {
            let mut id = String::new();
            let mut monkey = Monkey {
                num: None,
                op: None,
                humn_derived: HumnDerived::No,
            };

            if let Some(cap) = re_num.captures(&line) {
                id = cap[1].to_string();
                let num: i64 = cap[2].parse().unwrap();

                monkey.num = Some(num);
                if id.eq("humn") {
                    monkey.humn_derived = HumnDerived::Num;
                }
            } else if let Some(cap) = re_op.captures(&line) {
                id = cap[1].to_string();
                let param1 = cap[2].to_string();
                let op = cap[3].chars().next().unwrap();
                let param2 = cap[4].to_string();

                monkey.op = Some(MonkeyOp {
                    param1: param1,
                    param2: param2,
                    op: op,
                });
            } else {
                panic!("no regex matched");
            }
            monkeys.insert(id, monkey);
        }

        queue.push_back("root".to_string());
        queued.insert("root".to_string());

        loop {
            let id = queue.pop_front().unwrap();
            let monkey = monkeys.get(&id).unwrap();
            if let Some(num) = monkey.num {
                if id.eq("root") {
                    println!("root's number is: {}", num);
                    break;
                }
                println!("monkey {}'s number is: {}", id, num);
                continue;
            } else if let Some(op) = &monkey.op {
                let par1 = monkeys.get(&op.param1).unwrap();
                let par2 = monkeys.get(&op.param2).unwrap();

                if let (Some(num1), Some(num2)) = (par1.num, par2.num) {
                    let result = match op.op {
                        '+' => num1 + num2,
                        '-' => num1 - num2,
                        '*' => num1 * num2,
                        '/' => num1 / num2,
                        _ => todo!("unknown operator {}", op.op),
                    };
                    let mut humn_derived = HumnDerived::No;
                    if par1.humn_derived != HumnDerived::No {
                        humn_derived = HumnDerived::Left;
                    }
                    if par2.humn_derived != HumnDerived::No {
                        if humn_derived != HumnDerived::No {
                            panic!("both sides of monkey {} are humn derived", id);
                        }
                        humn_derived = HumnDerived::Right;
                    }
                    println!("calculated monkey {} num as {} (human derived: {:?})",
                        id, result, humn_derived);
                    let mut monkey = monkeys.get_mut(&id).unwrap();
                    monkey.num = Some(result);
                    monkey.humn_derived = humn_derived;
                    if id.eq("root") {
                        println!("root's number is: {}", result);
                        break;
                    }
                    continue;
                }
                if par1.num == None && !queued.contains(&op.param1) {
                    queue.push_back(op.param1.to_string());
                    queued.insert(op.param1.to_string());
                }
                if par2.num == None && !queued.contains(&op.param2) {
                    queue.push_back(op.param2.to_string());
                    queued.insert(op.param2.to_string());
                }
                queue.push_back(id);
            } else {
                panic!("monkey {id} has no num nor op");
            }
        }

let root = monkeys.get("root").unwrap();
        let mut target = root.num.unwrap();
        let root_op = root.op.as_ref().unwrap();
        let mut next = match root.humn_derived {
            HumnDerived::Left => &root_op.param1,
            HumnDerived::Right => &root_op.param2,
            _ => { panic!("root's humn_derived is {:?}", root.humn_derived); }
        };
        if root.humn_derived == HumnDerived::Left {
            target = monkeys.get(&root_op.param2).unwrap().num.unwrap();
        } else {
            target = monkeys.get(&root_op.param1).unwrap().num.unwrap();
        }

        loop {
            let monkey = monkeys.get(next).unwrap();
            if monkey.humn_derived == HumnDerived::Num {
                println!("humn should yell: {}", target);
                break;
            }
            let monkey_op = monkey.op.as_ref().unwrap();
            if monkey.humn_derived == HumnDerived::Left {
                next = &monkey_op.param1;
                let right_num = monkeys.get(&monkey_op.param2).unwrap().num.unwrap();
                target = match monkey_op.op {
                    '+' => target - right_num,
                    '-' => target + right_num,
                    '*' => target / right_num,
                    '/' => target * right_num,
                    _ => { panic!(""); },
                };
            } else {
                next = &monkey_op.param2;
                let left_num = monkeys.get(&monkey_op.param1).unwrap().num.unwrap();
                target = match monkey_op.op {
                    '+' => target - left_num,
                    '-' => left_num - target,
                    '*' => target / left_num,
                    '/' => left_num / target,
                    _ => { panic!(""); },
                };
            }
        }
    }
}

use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec::Vec;
use std::cmp::max;
use std::collections::HashMap;
use regex::Regex;

// copy/paste from Rust By Example
// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug)]
struct Valve {
    rate: i32,
    tunnels: Vec<String>,
}

#[derive(Debug)]
struct ValveState {
    score: i32,
    score2: i32,
    opened: bool,
}

const MINUTES: i32 = 30;

fn visit(valves: &HashMap<String, Valve>, states: &mut HashMap<String, ValveState>, code: &str, open: bool, minute: i32, score: i32) -> i32 {
    let valve = valves.get(code).unwrap();
    let state = states.get_mut(code).unwrap();
    let mut new_score = score;
    let mut max_score = score;
    let already_opened = state.opened;

    let old_score = state.score;
    if !open && old_score >= score {
        return max_score;
    }

//    println!("visited {code} in min {minute}, score {score} open {open}");
    if minute == MINUTES {
        return max_score;
    }


    if open {
        state.opened = true;
        new_score += (MINUTES - minute) * valve.rate;
    }

    state.score = new_score;

    for tun in &valve.tunnels {
        let mut do_open = false;
        if tun.eq(code) {
            if open {
                // just opened the valve, not visiting it againg
                continue;
            } else if valve.rate == 0 || already_opened {
                // already opened or not worth it
                continue;
            }
            do_open = true;
        }
        let visit_score = visit(valves, states, &tun, do_open, minute + 1, new_score);
        max_score = max(max_score, visit_score);
    }

    let state = states.get_mut(code).unwrap();
    state.score = old_score;
    if open {
        state.opened = false;
    }

    max_score
}

fn dual_visit(valves: &HashMap<String, Valve>, states: &mut HashMap<String, ValveState>,
              code1: &str, open1: bool, code2: &str, open2: bool, minute: i32, score: i32,
              mut num_to_open: i32) -> i32 {
    let valve1 = valves.get(code1).unwrap();
    let valve2 = valves.get(code2).unwrap();
    let mut new_score = score;
    let mut max_score = score;

    let state1 = states.get(code1).unwrap();
    let already_opened1 = state1.opened;
    let state2 = states.get(code2).unwrap();
    let already_opened2 = state2.opened;

    let old_score1 = state1.score;
    let old_score2 = state2.score2;
    if (!open1 && old_score1 >= score) || (!open2 && old_score2 >= score) {
        return max_score;
    }

//    println!("visited {code1},{code2} in min {minute}, score {score} to_open {num_to_open} open {open1},{open2}");
    if minute == MINUTES {
        return max_score;
    }

    // can't be both opening the same valve
    if code1.eq(code2) && open1 && open2 {
        return max_score;
    }

    if open1 {
        new_score += (MINUTES - minute) * valve1.rate;
    }
    if open2 {
        new_score += (MINUTES - minute) * valve2.rate;
    }

    let state1 = states.get_mut(code1).unwrap();
    if open1 {
        state1.opened = true;
        num_to_open -= 1;
    }
    state1.score = new_score;

    let state2 = states.get_mut(code2).unwrap();
    if open2 {
        state2.opened = true;
        num_to_open -= 1;
    }
    state2.score2 = new_score;

    if num_to_open > 0 {

    for tun1 in &valve1.tunnels {
        let mut do_open1 = false;
        if tun1.eq(code1) {
            if open1 {
                // just opened the valve, not visiting it againg
                continue;
            } else if valve1.rate == 0 || already_opened1 {
                // already opened or not worth it
                continue;
            }
            do_open1 = true;
        }
        for tun2 in &valve2.tunnels {
            let mut do_open2 = false;
            if tun2.eq(code2) {
                if open2 {
                    // just opened the valve, not visiting it againg
                    continue;
                } else if valve2.rate == 0 || already_opened2 {
                    // already opened or not worth it
                    continue;
                }
                do_open2 = true;
            }
            let visit_score = dual_visit(valves, states, &tun1, do_open1, &tun2, do_open2, minute + 1, new_score, num_to_open);
            max_score = max(max_score, visit_score);
        }
    }

    } else {
        if minute < 15 {
           // println!("opened all valves in min {minute}");
        }
        max_score = max(max_score, new_score);
    }

    let state1 = states.get_mut(code1).unwrap();
    state1.score = old_score1;
    if open1 {
        state1.opened = false;
    }

    let state2 = states.get_mut(code2).unwrap();
    state2.score2 = old_score2;
    if open2 {
        state2.opened = false;
    }

    max_score
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let mut valves: HashMap<String, Valve> = HashMap::new();
    let mut states: HashMap<String, ValveState> = HashMap::new();

    // Valve II has flow rate=0; tunnels lead to valves AA, JJ
    let re = Regex::new(r"Valve ([A-Z]+) has flow rate=(\d+); tunnels? leads? to valves? (.*)").unwrap();

    let mut num_to_open = 0;

    if let Ok(lines) = read_lines(&file_path) {
        let lines = lines.map(|l| l.unwrap());
        for line in lines {
            println!("{line}");
            let cap = re.captures(&line).unwrap();
            let code = &cap[1];
            let rate: i32 = cap[2].parse().unwrap();
            let tunnels = &cap[3];

            let mut tunvec: Vec<String> = Vec::new();
            tunvec.push(code.to_string());
            for t in tunnels.split(",") {
                tunvec.push(t.trim().to_string());
            }

            let valve = Valve {
                rate: rate,
                tunnels: tunvec,
            };
            let state = ValveState {
                score: -1,
                score2: -1,
                opened: false,
            };
            valves.insert(code.to_string(), valve);
            states.insert(code.to_string(), state);
            if rate > 0 {
                num_to_open += 1;
            }
        }
    }
    println!("{:?}", valves);
    let max_score = visit(&valves, &mut states, "AA", false, 0, 0);
    println!("max release pressure: {max_score}");
    println!("{:?}", valves);
    println!("{:?}", states);
    let max_score2 = dual_visit(&valves, &mut states, "AA", false, "AA", false, 4, 0, num_to_open);
    println!("max release pressure with elephant: {max_score2}");
}

use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec::Vec;
use std::collections::VecDeque;
use std::cmp::max;
use std::collections::HashMap;
use itertools::Itertools;
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

const MINUTES: i32 = 30;

struct Data {
    valves: HashMap<String, Valve>,
    v2i: HashMap<String, usize>,
    num_to_open: usize,
    v2v_distances: Vec<Vec<i32>>,
    rates: Vec<i32>,
}

impl Data {
    fn visit(&mut self, from: usize, to: usize, mut minutes: i32, mut todo: u64) -> i32 {
        let dist = self.v2v_distances[from][to] + 1;
        if minutes < dist {
            return 0;
        }
        minutes -= dist;
        let my_press = minutes * self.rates[to];
        let mut max_press = 0;
        todo &= !(1u64 << to);
        if todo > 0 {
            for vi in 1..=self.num_to_open {
                if (todo & (1u64 << vi)) > 0 {
                    let nested_press = self.visit(to, vi, minutes, todo);
                    max_press = max(max_press, nested_press);
                }
            }
        }

        max_press + my_press
    }

    fn determine_pressure(&mut self, minutes: i32, todo: u64) -> i32 {
        let mut press = 0;
        for vi in 1..=self.num_to_open {
            if (todo & 1u64 << vi) == 0 {
                continue;
            }
            let new_press = self.visit(0, vi, minutes, todo);
            press = max(press, new_press);
        }
        press
    }

    fn determine_pressure2(&mut self, minutes: i32, mut todo: u64) -> i32 {
        let mut max_press = 0;
        todo >>= 1;
        for t in 0..=((todo+1) >> 1) {
            let press1 = self.determine_pressure(minutes, t << 1);
            let press2 = self.determine_pressure(minutes, (todo - t) << 1);
            max_press = max(max_press, press1 + press2)
        }

        max_press
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let mut data = Data {
        valves: HashMap::new(),
        v2i: HashMap::new(),
        num_to_open: 0,
        v2v_distances: Vec::new(),
        rates: Vec::new(),
    };

    // Valve II has flow rate=0; tunnels lead to valves AA, JJ
    let re = Regex::new(r"Valve ([A-Z]+) has flow rate=(\d+); tunnels? leads? to valves? (.*)").unwrap();
    let mut todo: u64 = 0u64;

    data.v2i.insert("AA".to_string(), 0);
    data.rates.push(0);
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
            data.valves.insert(code.to_string(), valve);
            if rate > 0 && !code.eq("AA") {
                data.num_to_open += 1;
                data.v2i.insert(code.to_string(), data.num_to_open);
                data.rates.push(0);
                todo += 1u64 << data.num_to_open
            }
        }
    }
    println!("{:?}", data.valves);
    println!("{:?}", data.v2i);

    for _ in 0..=data.num_to_open {
        data.v2v_distances.push(vec![0; 1 + data.num_to_open]);
    }
    for (valve_code, valve_index) in &data.v2i {
        let mut distances: HashMap<&str, i32> = HashMap::new();
        let mut to_visit: VecDeque<(&str, i32)> = VecDeque::new();
        to_visit.push_back((valve_code, 0));
        while !to_visit.is_empty() {
            let (code, dist) = to_visit.pop_front().unwrap();
            if !distances.contains_key(code) {
                distances.insert(code, dist);
                for tun in &data.valves[code].tunnels {
                    to_visit.push_back((&tun, dist+1));
                }
            }
        }
        println!("distances from {valve_code}: {:?}", distances);
        let distvec = &mut data.v2v_distances[*valve_index];
        for (vc, vi) in &data.v2i {
            distvec[*vi] = *distances.get(vc as &str).unwrap();
        }
        data.rates[*valve_index] = data.valves[valve_code].rate;
    }
    println!("{:?}", data.v2v_distances);

    let max_pressure = data.determine_pressure(MINUTES, todo);
    println!("max pressure: {max_pressure}");

    let max_pressure2 = data.determine_pressure2(MINUTES-4, todo);
    println!("max presssure with elephant: {max_pressure2}");
}

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

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let mut valves: HashMap<String, Valve> = HashMap::new();
    let mut v2i: HashMap<String, usize> = HashMap::new();

    // Valve II has flow rate=0; tunnels lead to valves AA, JJ
    let re = Regex::new(r"Valve ([A-Z]+) has flow rate=(\d+); tunnels? leads? to valves? (.*)").unwrap();

    let mut num_to_open = 0;

    v2i.insert("AA".to_string(), 0);
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
            valves.insert(code.to_string(), valve);
            if rate > 0 && !code.eq("AA") {
                num_to_open += 1;
                v2i.insert(code.to_string(), num_to_open);
            }
        }
    }
    println!("{:?}", valves);
    println!("{:?}", v2i);
    let mut v2v_distances: Vec<Vec<i32>> = Vec::new();
    let mut rates = vec![0; 1 + num_to_open];
    for _ in 0..=num_to_open {
        v2v_distances.push(vec![0; 1 + num_to_open]);
    }
    for (valve_code, valve_index) in &v2i {
        let mut distances: HashMap<&str, i32> = HashMap::new();
        let mut to_visit: VecDeque<(&str, i32)> = VecDeque::new();
        to_visit.push_back((valve_code, 0));
        while !to_visit.is_empty() {
            let (code, dist) = to_visit.pop_front().unwrap();
            if !distances.contains_key(code) {
                distances.insert(code, dist);
                for tun in &valves[code].tunnels {
                    to_visit.push_back((&tun, dist+1));
                }
            }
        }
        println!("distances from {valve_code}: {:?}", distances);
        let distvec = &mut v2v_distances[*valve_index];
        for (vc, vi) in &v2i {
            distvec[*vi] = *distances.get(vc as &str).unwrap();
        }
        rates[*valve_index] = valves[valve_code].rate;
    }
    println!("{:?}", v2v_distances);

    let mut max_pressure = 0;
    let start = if valves.get("AA").unwrap().rate > 0 { 0 } else { 1 };
    'ext: for perm_len in 1..=(num_to_open - (start-1)) {
        println!("Trying with permutations of len {perm_len}");
        for perm in (start..=num_to_open).permutations(perm_len) {
            //println!("{:?}", perm);
            let mut minutes = MINUTES;
            let mut pressure = 0;
            let mut prev = 0;
            for vi in perm {
                let dist = v2v_distances[prev][vi] + 1;
                minutes -= dist;
                if minutes <= 0 {
                    break;
                }
                prev = vi;
                pressure += minutes * rates[vi];
            }
            if pressure > max_pressure {
                max_pressure = pressure;
                println!("max pressure: {max_pressure}");
            }
            if minutes >= 0 && perm_len < (num_to_open - (start-1)) {
                println!("permutations not long enough");
                continue 'ext;
            }
        }
        break;
    }
    println!("max pressure: {max_pressure}");
}

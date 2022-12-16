use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec::Vec;
use std::cmp::{min, max};
use std::collections::HashSet;
use regex::Regex;

// copy/paste from Rust By Example
// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug)]
#[derive(Eq, Hash, PartialEq)]
struct Pos(i32, i32);

fn dist(pos1: &Pos, pos2: &Pos) -> u32 {
    pos1.0.abs_diff(pos2.0) + pos1.1.abs_diff(pos2.1)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let part1_y: i32 = args[2].parse().unwrap();
    let part2_xy: i32 = args[3].parse().unwrap();

    let mut beacons: HashSet<Pos> = HashSet::new();
    let mut sensors: Vec<(Pos, u32)> = Vec::new();
    let mut xmin = i32::MAX;
    let mut ymin = i32::MAX;
    let mut xmax = i32::MIN;
    let mut ymax = i32::MIN;

    let re = Regex::new(r"Sensor at x=([-]?\d+), y=([-]?\d+): closest beacon is at x=([-]?\d+), y=([-]?\d+)").unwrap();

    if let Ok(lines) = read_lines(&file_path) {
        //Sensor at x=20, y=1: closest beacon is at x=15, y=3
        let lines = lines.map(|l| l.unwrap());
        for line in lines {
            let cap = re.captures(&line).unwrap();
            let xs: i32 = cap[1].parse().unwrap();
            let ys: i32 = cap[2].parse().unwrap();
            let xb: i32 = cap[3].parse().unwrap();
            let yb: i32 = cap[4].parse().unwrap();

            let b = Pos(xb, yb);
            let s = Pos(xs, ys);
            let d = dist(&b, &s);

            let id = i32::try_from(d).unwrap();

            xmin = min(xmin, xs - id);
            ymin = min(ymin, ys - id);
            xmax = max(xmax, xs + id);
            ymax = max(ymax, ys + id);

            beacons.insert(b);
            sensors.push((s, d));
        }
    }

    let y = part1_y;
    let mut no_beacons = 0;
    for x in xmin..=xmax {
        let pos = Pos(x, y);
        if beacons.contains(&pos) {
            continue
        }
        for s in &sensors {
            if dist(&pos, &s.0) <= s.1 {
                no_beacons += 1;
                break
            }
        }
    }
    println!("cols with no beacons: {}", no_beacons);

    let xymax = part2_xy;
    for y in 0..=xymax {
        let mut x = 0;
        while x < xymax {
            let pos = Pos(x, y);
            if beacons.contains(&pos) {
                continue;
            }
            let mut step = 0;
            for s in &sensors {
                let d = dist(&pos, &s.0);
                if d <= s.1 {
                    let id = i32::try_from(s.1 - d + 1).unwrap();
                    step = max(step, id);
                }
            }
            if step == 0 {
                println!("beacon possible at x={}, y={}", x, y);
                println!("tuning frequency: {}", u64::try_from(x).unwrap() * 4000000u64 + u64::try_from(y).unwrap());
                step = 1;
            }
            x += step;
        }
    }
}

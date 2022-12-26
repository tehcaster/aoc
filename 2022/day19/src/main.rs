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

struct Blueprint {
    id: i32,
    ore_ore: i32,
    clay_ore: i32,
    obs_ore: i32,
    obs_clay: i32,
    geo_ore: i32,
    geo_obs: i32,

    ore_max: i32,
}

#[derive(Default, Clone)]
struct State {
    ore: i32,
    clay: i32,
    obs: i32,
    geo: i32,

    ore_rob: i32,
    clay_rob: i32,
    obs_rob: i32,
    geo_rob: i32,
/*
    ore_proj: i32,
    clay_proj: i32,
    obs_proj: i32,
*/
}

#[derive(PartialEq)]
enum BuildAction {
    OreRobot,
    ClayRobot,
    ObsRobot,
    GeoRobot,
    NoRobot,
}

impl State {
    fn check_action(&self, bp: &Blueprint, action: &BuildAction) -> bool {
        match action {
            BuildAction::OreRobot => self.ore >= bp.ore_ore && bp.ore_max > self.ore_rob,
            BuildAction::ClayRobot => self.ore >= bp.clay_ore && bp.geo_obs > self.obs_rob && bp.obs_clay > (self.clay_rob + (self.clay / bp.obs_clay)),
            BuildAction::ObsRobot => self.ore >= bp.obs_ore && self.clay >= bp.obs_clay && bp.geo_obs > self.obs_rob,
            BuildAction::GeoRobot => self.ore >= bp.geo_ore && self.obs >= bp.geo_obs,
            BuildAction::NoRobot => true,
        }
    }

    fn collect_stuff(&mut self) {
        self.ore += self.ore_rob;
        self.clay += self.clay_rob;
        self.obs += self.obs_rob;
        self.geo += self.geo_rob;
    }

    fn perform_action(&mut self, bp: &Blueprint, action: &BuildAction) {
        self.collect_stuff();
        match action {
            BuildAction::OreRobot => {
                self.ore -= bp.ore_ore;
//                self.ore_proj -= bp.ore_ore;
                self.ore_rob += 1;
//                self.ore_proj += min;
            },
            BuildAction::ClayRobot => {
                self.ore -= bp.clay_ore;
//                self.ore_proj -= bp.clay_ore;
                self.clay_rob += 1;
//                self.clay_proj += min;
            },
            BuildAction::ObsRobot => {
                self.ore -= bp.obs_ore;
//                self.ore_proj -= bp.obs_ore;
                self.clay -= bp.obs_clay;
//                self.clay_proj -= bp.obs_clay;
                self.obs_rob += 1;
//                self.obs_proj += min;
            },
            BuildAction::GeoRobot => {
                self.ore -= bp.geo_ore;
//                self.ore_proj -= bp.geo_ore;
                self.obs -= bp.geo_obs;
//                self.obs_proj -= bp.geo_obs;
                self.geo_rob += 1;
            }
            BuildAction::NoRobot => { },
        }
    }
}

fn tick(bp: &Blueprint, state: &State, min: i32, quality: &mut i32) {
    if min == 0 {
        *quality = max(*quality, state.geo);
        return;
    }

    for action in [BuildAction::GeoRobot, BuildAction::ObsRobot, BuildAction::OreRobot, BuildAction::ClayRobot, BuildAction::NoRobot] {
        if state.check_action(bp, &action) {
            let mut state = state.clone();
            state.perform_action(bp, &action);
            tick(bp, &state, min - 1, quality);

            // not sure if it's valid - it's not, as evidenced by part 2 sample blueprint 1
//            if action == BuildAction::GeoRobot || action == BuildAction::ObsRobot {
//                break;
//            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let minutes: i32 = args[2].parse().unwrap();
    let mut blueprints = i32::MAX;
    let mut quality_level = 0;
    let mut quality_multiply = 1;

    if args.len() > 3 {
        blueprints = args[3].parse().unwrap();
    }

    // Blueprint 1: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 13 clay. Each geode robot costs 3 ore and 7 obsidian.
    let re = Regex::new(r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.").unwrap();

    if let Ok(lines) = read_lines(&file_path) {
        //Sensor at x=20, y=1: closest beacon is at x=15, y=3
        let lines = lines.map(|l| l.unwrap());
        for line in lines {
            let cap = re.captures(&line).unwrap();

            let mut bp = Blueprint {
                id: cap[1].parse().unwrap(),
                ore_ore: cap[2].parse().unwrap(),
                clay_ore: cap[3].parse().unwrap(),
                obs_ore: cap[4].parse().unwrap(),
                obs_clay: cap[5].parse().unwrap(),
                geo_ore: cap[6].parse().unwrap(),
                geo_obs: cap[7].parse().unwrap(),
                ore_max: 0,
            };

            bp.ore_max = max(bp.clay_ore, max(bp.obs_ore, bp.geo_ore));

            let mut state = State::default();
            state.ore_rob = 1;

            let mut quality = 0;
            tick(&bp, &state, minutes, &mut quality);
            println!("blueprint {} quality {}", bp.id, quality);
            quality_level += bp.id * quality;
            if bp.id <= 3 {
                quality_multiply *= quality;
            }
            if bp.id == blueprints {
                break;
            }
        }
        println!("total quality level: {}", quality_level);
        println!("up to first 3 blueprints multiplied: {}", quality_multiply);
    }
}

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

impl State {
    fn collect_stuff(&mut self) {
        self.ore += self.ore_rob;
        self.clay += self.clay_rob;
        self.obs += self.obs_rob;
        self.geo += self.geo_rob;
    }
}

fn tick(bp: &Blueprint, state: &State, min: i32, quality: &mut i32) {
    if min == 1 {
        // too late to build anything, just collect obsidian and return
        *quality = max(*quality, state.geo + state.geo_rob);
        return;
    }

    let need_obs = bp.geo_obs > state.obs_rob;
    let need_clay = need_obs && bp.obs_clay > (state.clay_rob + (state.clay / bp.obs_clay));
    let need_ore = bp.ore_max > state.ore_rob;

    let mut new_state = state.clone();
    new_state.collect_stuff();

    let mut can_ore = false;
    let mut can_clay = false;
    let mut can_obs = false;
    let mut can_geo = false;

    if need_ore && state.ore >= bp.ore_ore {
        can_ore = true;
        new_state.ore -= bp.ore_ore;
        new_state.ore_rob += 1;
        tick(bp, &new_state, min - 1, quality);
        new_state.ore += bp.ore_ore;
        new_state.ore_rob -= 1;
    }

    if need_clay && state.ore >= bp.clay_ore {
        can_clay = true;
        new_state.ore -= bp.clay_ore;
        new_state.clay_rob += 1;
        tick(bp, &new_state, min - 1, quality);
        new_state.ore += bp.clay_ore;
        new_state.clay_rob -= 1;
    }

    if need_obs && state.ore >= bp.obs_ore && state.clay >= bp.obs_clay {
        can_obs = true;
        new_state.ore -= bp.obs_ore;
        new_state.clay -= bp.obs_clay;
        new_state.obs_rob += 1;
        tick(bp, &new_state, min - 1, quality);
        new_state.ore += bp.obs_ore;
        new_state.clay += bp.obs_clay;
        new_state.obs_rob -= 1;
    }

    if state.ore >= bp.geo_ore && state.obs >= bp.geo_obs {
        can_geo = true;
        new_state.ore -= bp.geo_ore;
        new_state.obs -= bp.geo_obs;
        new_state.geo_rob += 1;
        tick(bp, &new_state, min - 1, quality);
        new_state.ore += bp.geo_ore;
        new_state.obs += bp.geo_obs;
        new_state.geo_rob -= 1;
    }

    // we can build (or don't need) any robot up to first clay - do it
    if (can_ore || !need_ore) && can_clay && state.clay_rob == 0 {
        return;
    }

    // we can build (or don't need) any robot up to first obsidian - do it
    if (can_ore || !need_ore) && (can_clay || !need_clay) && can_obs && state.obs_rob == 0 {
        return;
    }

    // we can build (or don't need) any of the robot - do it
    if (can_ore || !need_ore) && (can_clay || !need_clay) && (can_obs || !need_obs) && can_geo {
        return;
    }

    tick(bp, &new_state, min - 1, quality);
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

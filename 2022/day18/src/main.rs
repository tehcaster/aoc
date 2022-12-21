use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec::Vec;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::cmp::{min,max};

// copy/paste from Rust By Example
// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let mut cubes: HashSet<(i8, i8, i8)> = HashSet::new();
    let mut steam: HashSet<(i8, i8, i8)> = HashSet::new();
    let mut steam_todo: VecDeque<(i8, i8, i8)> = VecDeque::new();

    let mut min_x = i8::MAX;
    let mut min_y = i8::MAX;
    let mut min_z = i8::MAX;
    let mut max_x = i8::MIN;
    let mut max_y = i8::MIN;
    let mut max_z = i8::MIN;

    let lines = read_lines(&file_path).unwrap();
    for line in lines {
        let line = line.unwrap();
        let mut xyz = line.split(",");
        let x: i8 = xyz.next().unwrap().parse().unwrap();
        let y: i8 = xyz.next().unwrap().parse().unwrap();
        let z: i8 = xyz.next().unwrap().parse().unwrap();
        cubes.insert((x,y,z));
        min_x = min(min_x, x-1);
        max_x = max(max_x, x+1);
        min_y = min(min_y, y-1);
        max_y = max(max_y, y+1);
        min_z = min(min_z, z-1);
        max_z = max(max_z, z+1);
    }

    println!("total volume is x={}..{}, y={}..{}, z={}..{}", min_x, max_x, min_y, max_y, min_z, max_z);

    steam.insert((min_x, min_y, min_z));
    steam_todo.push_back((min_x, min_y, min_z));

    while !steam_todo.is_empty() {
        let (x, y, z) = steam_todo.pop_front().unwrap();
        for (x, y, z) in [(x-1, y, z), (x+1, y, z), (x, y-1, z), (x, y+1, z), (x, y, z-1), (x, y, z+1)] {
            if x < min_x || x > max_x || y < min_y || y > max_y || z < min_z || z > max_z {
                continue;
            }
            if cubes.contains(&(x,y,z)) {
                continue;
            }
            if steam.contains(&(x,y,z)) {
                continue;
            }
            steam.insert((x, y, z));
            steam_todo.push_back((x, y, z));
        }
    }

    let mut surface = 0;
    let mut surface_outer = 0;
    for (x, y, z) in &cubes {
        let x = *x;
        let y = *y;
        let z = *z;
        for (x, y, z) in [(x-1, y, z), (x+1, y, z), (x, y-1, z), (x, y+1, z), (x, y, z-1), (x, y, z+1)] {
            if !cubes.contains(&(x,y,z)) {
                surface += 1;
                if steam.contains(&(x,y,z)) {
                    surface_outer += 1;
                }
            }
        }
    }
    println!("surface area: {}", surface);
    println!("outer surface area: {}", surface_outer);
}

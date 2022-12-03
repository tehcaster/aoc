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

fn item_to_prio(item: u8) -> u64{
    let pr: u8 = match item {
        b'a'..=b'z' => {
            item - b'a' + 1
        },
        b'A'..=b'Z' => {
            item - b'A' + 27
        },
        _ => {
            panic!("uknown value");
        },
    };
    u64::from(pr)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let mut prio: u64 = 0;
    let mut group_prio: u64 = 0;
    let mut line_num = 0;
    let mut common_group: HashSet<u8, RandomState> = HashSet::new();

    if let Ok(lines) = read_lines(&file_path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line) = line {
                let half = line.len() / 2;
                let line = line.as_bytes();
                let mut comp1: HashSet<u8, RandomState> = HashSet::new();
                for val in line[..half].iter() {
                    comp1.insert(*val);
                    if line_num % 3 == 0 {
                        common_group.insert(*val);
                    }
                }

                let mut comp2: HashSet<u8, RandomState> = HashSet::new();
                for val in line[half..].iter() {
                    comp2.insert(*val);
                    if line_num % 3 == 0 {
                        common_group.insert(*val);
                    }
                }

                let common = comp1.intersection(&comp2).next().expect("no common value");
                prio += item_to_prio(*common);

                if line_num % 3 != 0 {
                    let union: HashSet<_> = comp1.union(&comp2).collect();
                    common_group.retain(|&val| union.contains(&val));
                }
            }
            line_num += 1;
            if line_num % 3 == 0 {
                let common_item = common_group.drain().next().expect("no common group value");
                group_prio += item_to_prio(common_item);
            }
        }
    }
    println!("total prio: {prio}, total group prio: {group_prio}");
}

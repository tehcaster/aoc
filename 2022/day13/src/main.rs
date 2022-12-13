use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec::Vec;
//use std::collections::VecDeque;

// copy/paste from Rust By Example
// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug)]
enum PacketData {
    Num(u32),
    List(Vec<PacketData>),
}

fn parse_num(bytes: &[u8]) -> (PacketData, usize) {
    let mut pos = 0;
    let mut num: u32 = 0;
    loop {
        match bytes[pos] {
            b'0'..=b'9' => {
                num *= 10;
                num += u32::from(bytes[pos] - b'0');
                pos += 1;
                continue;
            },
            _ => {
                return (PacketData::Num(num), pos);
            }
        }
    }
}

fn parse_list(bytes: &[u8]) -> (PacketData, usize) {
    let mut pos = 0;
    let mut list = Vec::<PacketData>::new();
    loop {
        match bytes[pos] {
            b']' => {
                return (PacketData::List(list), pos + 2);
            },
            b',' => {
                pos += 1;
            },
            _ => {
                let (data, dpos) = parse(&bytes[pos..]);
                list.push(data);
                pos += dpos;
            }
        }
    }
}

fn parse(bytes: &[u8]) -> (PacketData, usize) {
    match bytes[0] {
        b'[' => parse_list(&bytes[1..]),
        b'0'..=b'9' => parse_num(&bytes[0..]),
        x => {
            panic!("unexpected token {x}");
        },
    }
}

fn wrap_num(num: u32) -> Vec<PacketData> {
    let mut list = Vec::<PacketData>::new();
    list.push(PacketData::Num(num));
    return list;
}

#[derive(PartialEq)]
enum MyCmp {
    True,
    False,
    Cont,
}

fn compare_list(left: &Vec<PacketData>, right: &Vec<PacketData>) -> MyCmp {
    let mut right_iter = right.iter();

    for l in left.iter() {
        match right_iter.next() {
            None => {
                return MyCmp::False;
            },
            Some(r) => {
                match compare(l, r) {
                    MyCmp::Cont => {
                        continue;
                    },
                    x => {
                        return x;
                    },
                }
            }
        }
    }
    match right_iter.next() {
        None => MyCmp::Cont,
        _ => MyCmp::True,
    }
}

fn compare(left: &PacketData, right: &PacketData) -> MyCmp {
    match (left, right) {
        (PacketData::Num(left), PacketData::Num(right)) => {
            if left < right {
                MyCmp::True
            } else if left == right {
                MyCmp::Cont
            } else {
                MyCmp::False
            }
        },
        (PacketData::List(left), PacketData::List(right)) => compare_list(left, right),
        (PacketData::Num(left), PacketData::List(right)) => {
            let left = wrap_num(*left);
            compare_list(&left, right)
        },
        (PacketData::List(left), PacketData::Num(right)) => {
            let right = wrap_num(*right);
            compare_list(left, &right)
        },
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let (divider1, _) = parse("[[2]]".as_bytes());
    let (divider2, _) = parse("[[6]]".as_bytes());

    let mut div1_idx = 1;
    let mut div2_idx = 2;

    if let Ok(lines) = read_lines(&file_path) {
        let mut lines = lines.map(|l| l.unwrap());
        let mut checksum = 0;
        let mut index = 1;
        loop {
            let left = lines.next().expect("left list");
            let right = lines.next().expect("right list");

            let (left, _) = parse(left.as_bytes());
            let (right, _) = parse(right.as_bytes());

            //println!("{:?}", left);
            //println!("{:?}", right);

            if compare(&left, &right) == MyCmp::True {
                //println!("pair {} is in order", index);
                checksum += index;
            }

            for sig in [&left, &right] {
                if compare(sig, &divider2) == MyCmp::True {
                    div2_idx += 1;
                    if compare(sig, &divider1) == MyCmp::True {
                        div1_idx += 1;
                    }
                }
            }
            index += 1;

            let empty = lines.next();
            if empty.is_none() {
                break;
            }
        }
        println!("checksum: {checksum}");
        println!("decoder key: {}", div1_idx * div2_idx);
    }
}

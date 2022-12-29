use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec::Vec;

// copy/paste from Rust By Example
// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug)]
struct Node {
    next: usize,
    prev: usize,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let mut nodes: Vec<Node> = Vec::new();

    let mut nums: Vec<i64> = Vec::new();

    let mut zero_id = 0;

    let mut key: i64 = 1;

    let mut rounds: usize = 1;

    if args.len() > 2 {
        key = args[2].parse().unwrap();
        rounds = args[3].parse().unwrap();
    }

    if let Ok(lines) = read_lines(&file_path) {
        // Consumes the iterator, returns an (Optional) String
        for (id, line) in lines.enumerate() {
            let line = line.unwrap();

            let num: i64 = line.parse().unwrap();

            let node = Node {
                next: id + 1,
                prev: if id == 0 { 0 } else { id - 1 },
            };

            nodes.push(node);
            nums.push(num);

            if num == 0 {
                zero_id = id;
            }
        }

        let len = nodes.len();

        let mut first_node = nodes.get_mut(0).unwrap();
        first_node.prev = len - 1;
        let mut last_node = nodes.get_mut(len - 1).unwrap();
        last_node.next = 0;

//        println!("{:?}", nodes);
//        println!("{:?}", nums);
//        println!("nodes: {}", nodes.len());
//        println!("nums: {}",nums.len());

        let num_nums: i64 = len.try_into().unwrap();
        for _ in 0..rounds {
          for (id, num) in nums.iter().enumerate() {
            let num = *num;

            if num == 0 || (num*key) % (num_nums-1) == 0 {
                continue;
            }
            let mut dest = id;
            if num > 0 {
                for _ in 0..((num*key) % (num_nums-1)) {
                    let node = nodes.get(dest).unwrap();
                    dest = node.next;
                }
            } else {
                for _ in 0..((num*key) % (num_nums-1)).abs()+1 {
                    let node = nodes.get(dest).unwrap();
                    dest = node.prev;
                }
            }
//            println!("moving {} to {}", num, dest);
            let src_node = nodes.get(id).unwrap();
            let prev = src_node.prev;
            let next = src_node.next;

            let mut prev_node = nodes.get_mut(prev).unwrap();
            prev_node.next = next;

            let mut next_node = nodes.get_mut(next).unwrap();
            next_node.prev = prev;

            let mut dest_node = nodes.get_mut(dest).unwrap();
            let next = dest_node.next;
            dest_node.next = id;

            let mut next_node = nodes.get_mut(next).unwrap();
            next_node.prev = id;

            let mut moved_node = nodes.get_mut(id).unwrap();
            moved_node.prev = dest;
            moved_node.next = next;

//            println!("{:?}", nodes);
          }
        }

        let mut id = zero_id;
        let mut sum = 0;
        for _ in 0..3 {
            for _ in 0..(1000 % num_nums) {
                let node = nodes.get(id).unwrap();
                id = node.next;
            }
            let num = nums[id];
            println!("1000th num is {}", num*key);
            sum += num*key;
        }
        println!("sum is {}", sum);
    }
}

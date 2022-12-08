use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec::Vec;
use std::collections::HashMap;

// copy/paste from Rust By Example
// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

struct MyFile(String, u64);

struct MyDir {
    subdirs: HashMap<String, MyDir>,
    files: Vec<MyFile>,
    size: u64,
    listed: bool,
}

impl MyDir {
    fn new() -> Self {
        Self {
            subdirs: HashMap::new(),
            files: Vec::new(),
            size: 0,
            listed: false,
        }
    }
}

// done like this for 'cd ..' because I don't know how to have mutable references to parent
// directories
fn find_dir<'a>(root: &'a mut MyDir, path: &Vec<String>) -> &'a mut MyDir {
    let mut dir = root;

    for dirname in path {
        dir = dir.subdirs.get_mut(dirname).expect("path backtracking failed");
    }

    dir
}

fn calc_size(dir: &MyDir, sum_sizes: &mut u64) -> u64 {
    let mut size = 0;
    for (_, subdir) in dir.subdirs.iter() {
        size += calc_size(subdir, sum_sizes);
    }
    size += dir.size;
    if size < 100000 {
        *sum_sizes += size;
    }
    size
}

fn calc_size2(dir: &MyDir, at_least: u64, best_match: &mut u64) -> u64 {
    let mut size = 0;
    for (_, subdir) in dir.subdirs.iter() {
        size += calc_size2(subdir, at_least, best_match);
    }
    size += dir.size;

    if size >= at_least && size < *best_match {
        *best_match = size;
    }

    size
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let mut listing_mode = false;
    let mut root_dir = MyDir::new();
    let mut cwd: &mut MyDir = &mut root_dir;
    let mut path: Vec<String> = Vec::new();

    if let Ok(lines) = read_lines(&file_path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line) = line {
                let mut line_iter = line.split(" ");
                let first = line_iter.next().expect("unexpected empty line");
                if first.eq("$") {
                    listing_mode = false;
                    let cmd = line_iter.next().expect("missing command");
                    if cmd.eq("ls") {
                        if cwd.listed {
                            panic!("trying to list already known dir");
                        }
                        listing_mode = true;
                    } else if cmd.eq("cd") {
                        let cdir = line_iter.next().expect("missing dir to cd into");
                        if cdir.eq("/") {
                            cwd = &mut root_dir;
                        } else if cdir.eq("..") {
                            path.pop();
                            cwd = find_dir(&mut root_dir, &path);
                        } else {
                            path.push(cdir.to_string());
                            cwd = cwd.subdirs.get_mut(cdir).expect("trying to cd into unknown subdirectory");
                        }
                    }
                } else {
                    if !listing_mode {
                        panic!("got listing not after ls command");
                    }
                    let name = line_iter.next().expect("missing file or dir name");
                    if first.eq("dir") {
                        cwd.subdirs.insert(name.to_string(), MyDir::new());
                    } else {
                        let size: u64 = first.parse().expect("file size not parseable");
                        cwd.files.push(MyFile(name.to_string(), size));
                        cwd.size += size;
                    }
                }
            }
        }
    }

    let mut sum_size = 0;
    let size = calc_size(&root_dir, &mut sum_size);
    println!("total size: {size}");
    println!("sum of <100000 sizes: {sum_size}");

    let mut best_match = size;
    calc_size2(&root_dir, size - 40000000, &mut best_match);
    println!("need to delete directory of size {best_match}");
}

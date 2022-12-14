use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec::Vec;
use std::cmp::{min, max};

// copy/paste from Rust By Example
// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

struct Grid {
    cols: Vec<Vec<u8>>,
    nrows: usize,
    ncols: usize,
    rocks_col_min: usize,
    rocks_col_max: usize,
    rocks_row_max: usize,
}

impl Grid {
    fn resize(&mut self, x: usize, y: usize) {
        if self.ncols <= x {
            for _ in self.ncols..(x+1)*2 {
                let mut vec = Vec::new();
                for _ in 0..self.nrows {
                    vec.push(b'.');
                }
                self.cols.push(vec);
            }
            self.ncols = (x+1)*2;
        }
        if self.nrows <= y {
            for col in self.cols.iter_mut() {
                for _ in self.nrows..(y+1)*2 {
                    col.push(b'.');
                }
            }
            self.nrows = (y+1)*2;
        }
    }

    fn add_floor(&mut self) {
        self.add_rocks(
            (0, self.rocks_row_max + 2),
            (2 * self.rocks_col_max, self.rocks_row_max + 2));
    }

    fn add_rocks(&mut self, prev: (usize, usize), next: (usize, usize)) {
        let (xp, yp) = prev;
        let (xn, yn) = next;

        let xmin = min(xp, xn);
        let xmax = max(xp, xn);
        let ymin = min(yp, yn);
        let ymax = max(yp, yn);

        self.resize(xmax, ymax);

        self.rocks_col_min = min(self.rocks_col_min, xmin);
        self.rocks_col_max = max(self.rocks_col_max, xmax);
        self.rocks_row_max = max(self.rocks_row_max, ymax);


        if xp == xn {
            for y in ymin..=ymax {
                self.cols[xp][y] = b'#';
            }
        } else if yp == yn {
            for x in xmin..=xmax {
                self.cols[x][yp] = b'#';
            }
        } else {
            panic!("rocks not in straight line");
        }
    }

    fn add_sand(&mut self, mut x: usize, mut y: usize) -> bool {
        if self.cols[x][y] != b'.' {
            return false;
        }
        loop {
            if y >= self.rocks_row_max {
                return false;
            }
            if self.cols[x][y+1] == b'.' {
                y += 1;
            } else if self.cols[x-1][y+1] == b'.' {
                x -= 1;
                y += 1;
            } else if self.cols[x+1][y+1] == b'.' {
                x += 1;
                y += 1;
            } else {
                self.cols[x][y] = b'o';
                return true;
            }
        }
    }

    fn print(&self) {
        for nrow in 0..=self.rocks_row_max {
            let mut line = String::new();
            for ncol in self.rocks_col_min..=self.rocks_col_max {
                let c = self.cols[ncol][nrow];
                line.push(char::from(c));
            }
            println!("{line}");
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let mut grid = Grid {
        cols: Vec::new(),
        nrows: 0,
        ncols: 0,
        rocks_col_min: usize::MAX,
        rocks_col_max: 0,
        rocks_row_max: 0,
    };

    if let Ok(lines) = read_lines(&file_path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line) = line {
                let mut prev = None;
                for xy in line.split("->") {
                    let mut xy = xy.trim().split(',');
                    let x: usize = xy.next().expect("x next").parse().expect("x parse");
                    let y: usize = xy.next().expect("y next").parse().expect("y parse");
                    let next = (x,y);

                    if let Some(prev) = prev {
                        grid.add_rocks(prev, next);
                    }

                    prev = Some(next);
                }
            }
        }
    }

    // uncomment for part 1;
    grid.add_floor();

//    grid.print();

//    println!("");

    let mut units = 0;
    while grid.add_sand(500, 0) {
        units += 1;
    }
//    grid.print();
    println!("units of sand until fall: {units}");
}

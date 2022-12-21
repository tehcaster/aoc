use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec::Vec;
use std::collections::VecDeque;

// copy/paste from Rust By Example
// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

const WIDTH: u8 = 7;
const INSERT_COL: u8 = 2;
const FLOOR: u8 = 127;

#[derive(Debug)]
struct Shape {
    rows: Vec<u8>,
    width: u8,
    height: usize,
}

#[derive(Debug)]
struct Field {
    rows: VecDeque<u8>,
    height: usize,
    rocks_height: usize,
    trimmed: usize,
}

impl Field {
    fn new() -> Self {
        let mut field = Field {
            rows: VecDeque::new(),
            height: 1,
            rocks_height: 1,
            trimmed: 0,
        };
        field.rows.push_back(FLOOR);
        field
    }
/*
    fn print(&self) {
        for row in self.rows.iter().rev() {
            let line = std::str::from_utf8(row).unwrap();
            println!("{}", line);
        }
    }
*/
    fn shape_fits(&self, shape: &Shape, shape_row: usize, shape_col: u8) -> bool {
        let colshift = WIDTH - shape_col - shape.width;
        for rownum in 0..shape.height {
            if (self.rows[shape_row+rownum-self.trimmed] & (shape.rows[rownum] << colshift)) != 0 {
                return false;
            }
        }
        true
    }

    fn place_shape(&mut self, shape: &Shape, shape_row: usize, shape_col: u8) {
        let colshift = WIDTH - shape_col - shape.width;
        for rownum in 0..shape.height {
            self.rows[shape_row+rownum-self.trimmed] |= shape.rows[rownum] << colshift;
        }
        if shape_row + shape.height > self.rocks_height {
            self.rocks_height = shape_row + shape.height;
        }
        if self.rocks_height > self.trimmed + 2000 {
            self.rows.drain(0..1000);
            self.trimmed += 1000;
        }
    }

    fn add_shapes(&mut self, shapes: &[Shape], jets: &str, mut remaining: u64) {
        let max_jetpos = jets.len();
        let mut jetpos = 0;
        let jet_bytes = jets.as_bytes();

        for shape in shapes.iter().cycle() {
//            println!("inserting shape: {:?}", shape);
            let mut shape_row = self.rocks_height + 3;
            let mut shape_col = INSERT_COL;
            let need_rows = shape_row + shape.height;
            for _ in self.height..need_rows {
                self.rows.push_back(0);
                self.height += 1;
            }
            let mut above_rocks = 3;
            loop {
                let jet = jet_bytes[jetpos % max_jetpos];
                jetpos += 1;
                let mut jet_col = shape_col;
                match jet {
                    b'<' => {
                        if shape_col >= 1 {
                            if above_rocks > 0 || self.shape_fits(shape, shape_row, shape_col - 1) {
                                jet_col = shape_col - 1;
                            }
                        }
                    },
                    b'>' => {
                        if shape_col + shape.width < WIDTH {
                            if above_rocks > 0 || self.shape_fits(shape, shape_row, shape_col + 1) {
                                jet_col = shape_col + 1;
                            }
                        }
                    },
                    _ => { todo!("unknown jet {}", jet); },
                };
                shape_col = jet_col;
//                println!("new col after jet: {shape_col}");
                if above_rocks > 0 || self.shape_fits(shape, shape_row - 1, shape_col) {
                    shape_row -= 1;
  //                  println!("new row after fall: {shape_row}");
                } else {
                    self.place_shape(shape, shape_row, shape_col);
                    break;
                }
                above_rocks -= 1;
            }
            remaining -= 1;
            if remaining == 0 {
                break;
            }
        }
    }
}

impl Shape {
    fn new(arr: &[&str]) -> Self {
        let mut shape = Shape {
            rows: Vec::new(),
            width: 0,
            height: 0,
        };
        for row in arr {
            let mut val: u8 = 0;
            let vec: Vec<u8> = Vec::from_iter(row.bytes());
            shape.width = u8::try_from(vec.len()).unwrap();
            for b in vec {
                val <<= 1;
                if b == b'#' {
                    val |= 1;
                }
            }
            shape.rows.push(val);
            shape.height += 1;
        }
        shape.rows.reverse();
        shape
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let num_shapes: u64 = args[2].parse().unwrap();
    let shapes = [Shape::new(&["####"]),
        Shape::new(&[".#.", "###", ".#."]),
        Shape::new(&["..#", "..#", "###"]),
        Shape::new(&["#", "#", "#", "#"]),
        Shape::new(&["##", "##"])];

    println!("{:?}", shapes);

    let mut lines = read_lines(&file_path).unwrap();
    let line = lines.next().unwrap().unwrap();
    println!("{}", line);

    let mut field = Field::new();

    field.add_shapes(&shapes, &line, num_shapes);
//    field.print();
    println!("tower height: {}", field.rocks_height - 1);
//    println!("{:?}", field);



}

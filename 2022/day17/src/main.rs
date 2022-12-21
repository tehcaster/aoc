use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec::Vec;
use std::collections::HashMap;
use std::collections::HashSet;

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
const SIZE: usize = 2048;
const SNAPSIZE: usize = 20;

#[derive(Debug)]
struct Shape {
    rows: Vec<u8>,
    width: u8,
    height: usize,
    down_move_height: usize,
}

#[derive(Debug)]
struct Field {
    height: usize,
    rocks_height: usize,
}

impl Field {
    fn new() -> Self {
        let field = Field {
            height: 1,
            rocks_height: 1,
        };
        field
    }

    fn shape_fits(&self, shape: &Shape, shape_row: usize, shape_col: u8, rows: &[u8]) -> bool {
        let colshift = WIDTH - shape_col - shape.width;
        for rownum in 0..shape.height {
            if (rows[(shape_row + rownum) % SIZE] & (shape.rows[rownum] << colshift)) != 0 {
                return false;
            }
        }
        true
    }

    // true if fits at shape_row - 1; false if not; placed on shape_row
    fn try_move_down_shape(&mut self, shape: &Shape, shape_row: usize, shape_col: u8, rows: &mut [u8]) -> bool {
        let colshift = WIDTH - shape_col - shape.width;
        let mut ret = true;
        for rownum in 0..shape.down_move_height {
            if (rows[(shape_row - 1 + rownum) % SIZE] & (shape.rows[rownum] << colshift)) != 0 {
                ret = false;
                break;
            }
        }
        if ret {
            return true;
        }
        for rownum in 0..shape.height {
            rows[(shape_row + rownum) % SIZE] |= shape.rows[rownum] << colshift;
        }
        if shape_row + shape.height > self.rocks_height {
            self.rocks_height = shape_row + shape.height;
        }
        false
    }

    fn add_shapes(&mut self, shapes: &[Shape], jets: &str, num_shapes: usize) {
        let mut remaining = num_shapes;
        let max_jetpos = jets.len();
        let jet_bytes = jets.as_bytes();
        let mut rows = [0u8; SIZE];
        let mut jetpos_shape_set: HashSet<(usize, usize)> = HashSet::new();
        let mut jetpos_shape_vec_set: HashMap<(usize, usize, Vec<u8>), (usize, usize)> = HashMap::new(); 
        let mut prev_snap: Option<Vec<u8>> = None;
        let mut looking_for_cycles = true;
        let mut skipped_rocks_height: usize = 0;

//        println!("shapes: {}, jets: {}, shapes*jets: {}", shapes.len(), max_jetpos, shapes.len()*max_jetpos);

        rows[0] = FLOOR;

        let mut jetpos: usize = 0;
        let prev_remaining = remaining;
        let rocks_height_prev = self.rocks_height;
        for shape in shapes.iter().cycle() {
//            println!("inserting shape: {:?}", shape);
//            println!("{}", rows[0]);
            let mut shape_row = self.rocks_height + 3;
            let mut shape_col = INSERT_COL;
            let need_rows = shape_row + shape.height;
            for i in self.height..need_rows {
                rows[i % SIZE] = 0;
                self.height += 1;
            }
            let mut above_rocks = 3;
            loop {
//                println!("{}", rows[0]);
                let jet = jet_bytes[jetpos % max_jetpos];
                jetpos += 1;
                let mut jet_col = shape_col;
                match jet {
                    b'<' => {
                        if shape_col >= 1 {
                            if above_rocks > 0 || self.shape_fits(shape, shape_row, shape_col - 1, &rows) {
                                jet_col = shape_col - 1;
                            }
                        }
                    },
                    b'>' => {
                        if shape_col + shape.width < WIDTH {
                            if above_rocks > 0 || self.shape_fits(shape, shape_row, shape_col + 1, &rows) {
                                jet_col = shape_col + 1;
                            }
                        }
                    },
                    _ => { todo!("unknown jet {}", jet); },
                };
                shape_col = jet_col;
//                println!("new col after jet: {shape_col}");
                if above_rocks > 0 || self.try_move_down_shape(shape, shape_row, shape_col, &mut rows) {
                    shape_row -= 1;
                    above_rocks -= 1;
//                    println!("new row after fall: {shape_row}");
                } else {
                    break;
                }
            }
            remaining -= 1;
            if remaining == 0 {
                break;
            }

            if !looking_for_cycles {
                continue;
            }

            let shape_num = (num_shapes - remaining) % shapes.len();
            let jet_num = jetpos % max_jetpos;
            if jetpos_shape_set.contains(&(jet_num, shape_num)) {
                println!("possible cycle at {} {}", jet_num, shape_num);
                let mut snapvec: Vec<u8> = Vec::new();
                for i in 0..SNAPSIZE {
                    snapvec.push(rows[(self.rocks_height - SNAPSIZE + i) % SIZE]);
                }
                let key = (jet_num, shape_num, snapvec);
                if jetpos_shape_vec_set.contains_key(&key) {
                    let (old_rocks_height, old_remaining) = jetpos_shape_vec_set.get(&key).unwrap();
                    let cycle_shapes = old_remaining - remaining;
                    let cycle_rocks = self.rocks_height - old_rocks_height;
                    println!("cycle found! {} shapes {} rock_height", cycle_shapes, cycle_rocks);
                    let num_cycles = remaining / cycle_shapes;
                    skipped_rocks_height = num_cycles * cycle_rocks;
                    remaining -= num_cycles * cycle_shapes;
                    println!("fast forward {} cycles: {} shapes {} rock_height", num_cycles, num_cycles * cycle_shapes, skipped_rocks_height);
                    looking_for_cycles = false;
                    if remaining == 0 {
                        break;
                    }
                } else {
                    jetpos_shape_vec_set.insert(key, (self.rocks_height, remaining));
                }
            } else {
                jetpos_shape_set.insert((jet_num, shape_num));
            }
        }
        self.rocks_height += skipped_rocks_height;
    }
}

impl Shape {
    fn new(arr: &[&str], down_move_height: usize) -> Self {
        let mut shape = Shape {
            rows: Vec::new(),
            width: 0,
            height: 0,
            down_move_height: down_move_height,
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
    let num_shapes: usize = args[2].parse().unwrap();
    let shapes = [Shape::new(&["####"], 1),
        Shape::new(&[".#.", "###", ".#."], 2),
        Shape::new(&["..#", "..#", "###"], 1),
        Shape::new(&["#", "#", "#", "#"], 1),
        Shape::new(&["##", "##"], 1)];

//    println!("{:?}", shapes);

    let mut lines = read_lines(&file_path).unwrap();
    let line = lines.next().unwrap().unwrap();
//    println!("{}", line);

    let mut field = Field::new();

    field.add_shapes(&shapes, &line, num_shapes);
//    field.print();
    println!("tower height: {}", field.rocks_height - 1);
//    println!("{:?}", field);

}

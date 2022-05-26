use aoc21::load_data;
use std::vec::Vec;

struct Depth {
    depth: u32,
}

impl Depth {
    fn new(s: &str) -> Depth {
        let depth: u32 = s.parse().unwrap();

        Depth { depth }
    }
}

fn solve_part_1(vec: &Vec<Depth>) {
    let mut num_increases = 0;
    let mut last_value = std::u32::MAX;
    for depth in vec {
        if depth.depth > last_value {
            num_increases += 1;
        }
        last_value = depth.depth;
    }
    println!("Found {} increasing value pairs.", num_increases);
}

fn solve_part_2(vec: &Vec<Depth>) {
    let mut num_increases = 0;
    let mut a: u32 = 0;
    let mut b: u32 = 0;
    let mut c: u32 = 0;
    for idx in 0..vec.len() {
        let cur_depth = vec[idx].depth;
        if idx % 3 == 0 {
            b += cur_depth;
            c += cur_depth;
            if b > a && idx > 0 {
                num_increases += 1;
            }
            a = cur_depth;
        } else if idx % 3 == 1 {
            a += cur_depth;
            c += cur_depth;

            if c > b && idx > 1 {
                num_increases += 1;
            }
            b = cur_depth;
        } else {
            a += cur_depth;
            b += cur_depth;
            if a > c && idx > 2 {
                num_increases += 1;
            }
            c = cur_depth;
        }
    }

    println!("Found {} increasing value pairs.", num_increases);
}

fn main() {
    let filename = "data/day01/input.txt";

    let vec = load_data(filename, Depth::new);
    solve_part_1(&vec);
    solve_part_2(&vec);
}

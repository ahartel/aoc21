use std::vec::Vec;
use aoc21::load_data;


struct BinValue {
    value: String,
}

impl BinValue {
    fn new(s : &str) -> BinValue {
        BinValue {
            value: String::from(s),
        }
    }
}

fn solve_part_1(vec : &Vec<BinValue>) {
    let num_digits = vec[0].value.len();
    let mut ones  = vec![0; num_digits];
    let mut zeros = vec![0; num_digits];

    for bit_pos in 0..num_digits {
        for c in vec {
            let bit = c.value.as_str().chars().nth(bit_pos).unwrap();
            if bit == '0' {
                zeros[bit_pos] += 1;
            }
            else {
                ones[bit_pos] += 1;
            }
        }
    }
    println!("ones : {:?}", ones);
    println!("zeros: {:?}", zeros);

    let mut epsilon : usize = 0;
    let mut gamma : usize = 0;
    for bit_pos in 0..num_digits {
        let o = ones[num_digits-1-bit_pos];
        let z = zeros[num_digits-1-bit_pos];
        if o > z {
            gamma |= 1 << bit_pos;
        }
        else {
            epsilon |= 1 << bit_pos;
        }
    }
    println!("gamma {}, epsilon {}, Product {}",
        gamma,
        epsilon,
        gamma * epsilon);
}

fn solve_part_2(_vec : &Vec<BinValue>) {
    // let mut aim : i32 = 0;
    // let mut postition : i32 = 0;
    // let mut depth : i32 = 0;
    // for c in vec {
    //     match c.command.as_str() {
    //         "forward" => { postition += c.amount; depth += aim * c.amount; },
    //         "down" => aim += c.amount,
    //         "up" => aim -= c.amount,
    //         &_ => panic!("Reached unreachable code")
    //     }
    // }
    // println!("Horizontal postition {}, Depth {}, Product {}",
    //     postition,
    //     depth,
    //     postition * depth);
}

fn main() {
    let filename = "data/day03/input.txt";

    let vec = load_data(filename, BinValue::new);
    solve_part_1(&vec);
    solve_part_2(&vec);
}
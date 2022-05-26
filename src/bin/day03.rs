use aoc21::load_data;
use std::vec::Vec;

#[derive(Clone)]
struct BinValue {
    value: String,
}

impl BinValue {
    fn new(s: &str) -> BinValue {
        BinValue {
            value: String::from(s),
        }
    }
}

fn count_occurences_of_bits(vec: &Vec<BinValue>) -> (Vec<usize>, Vec<usize>) {
    let num_digits = vec[0].value.len();
    let mut ones = vec![0; num_digits];
    let mut zeros = vec![0; num_digits];

    for bit_pos in 0..num_digits {
        for c in vec {
            let bit = c.value.as_str().chars().nth(bit_pos).unwrap();
            if bit == '0' {
                zeros[bit_pos] += 1;
            } else {
                ones[bit_pos] += 1;
            }
        }
    }
    (ones, zeros)
}

fn solve_part_1(vec: &Vec<BinValue>) {
    let num_digits = vec[0].value.len();
    let (ones, zeros) = count_occurences_of_bits(vec);
    println!("ones : {:?}", ones);
    println!("zeros: {:?}", zeros);

    let mut epsilon: usize = 0;
    let mut gamma: usize = 0;
    for bit_pos in 0..num_digits {
        let o = ones[num_digits - 1 - bit_pos];
        let z = zeros[num_digits - 1 - bit_pos];
        if o > z {
            gamma |= 1 << bit_pos;
        } else {
            epsilon |= 1 << bit_pos;
        }
    }
    println!(
        "gamma {}, epsilon {}, Product {}",
        gamma,
        epsilon,
        gamma * epsilon
    );
}

fn my_filter(value: &String, bit_pos: usize, more_ones: bool) -> bool {
    let character = value.chars().nth(bit_pos).unwrap();
    let matches: bool = (more_ones && character == '1') || (!more_ones && character == '0');
    println!(
        "value: {}, bit_pos: {}, more_ones: {}, match: {}",
        value, bit_pos, more_ones, matches
    );
    matches
}

fn solve_part_2(vec: &Vec<BinValue>) {
    let o2 = {
        let mut bit_pos = 0;
        let mut found = vec.len();
        let mut from_vec = vec.clone();
        let mut to_vec: Vec<BinValue> = Vec::new();
        while found > 1 {
            let (ones, zeros) = count_occurences_of_bits(&from_vec);
            println!("ones : {:?}", ones);
            println!("zeros: {:?}", zeros);

            for bin_value in from_vec {
                if my_filter(&bin_value.value, bit_pos, ones[bit_pos] >= zeros[bit_pos]) {
                    to_vec.push(BinValue {
                        value: bin_value.value,
                    });
                }
            }
            found = to_vec.len();
            from_vec = to_vec.clone();
            to_vec.clear();
            bit_pos += 1;
        }

        let o2 = isize::from_str_radix(from_vec[0].value.as_str(), 2).unwrap();
        o2
    };
    let co2 = {
        let mut bit_pos = 0;
        let mut found = vec.len();
        let mut from_vec = vec.clone();
        let mut to_vec: Vec<BinValue> = Vec::new();
        while found > 1 {
            let (ones, zeros) = count_occurences_of_bits(&from_vec);
            println!("ones : {:?}", ones);
            println!("zeros: {:?}", zeros);

            for bin_value in from_vec {
                if my_filter(&bin_value.value, bit_pos, ones[bit_pos] < zeros[bit_pos]) {
                    to_vec.push(BinValue {
                        value: bin_value.value,
                    });
                }
            }
            found = to_vec.len();
            from_vec = to_vec.clone();
            to_vec.clear();
            bit_pos += 1;
        }

        let co2 = isize::from_str_radix(from_vec[0].value.as_str(), 2).unwrap();
        co2
    };
    println!("o2: {}, co2: {}, product: {}", o2, co2, o2 * co2);
}

fn main() {
    let filename = "data/day03/input.txt";

    let vec = load_data(filename, BinValue::new);
    solve_part_1(&vec);
    solve_part_2(&vec);
}

use std::vec::Vec;
use aoc21::load_data;
use std::collections::HashMap;


struct Line {
    wires: Vec<String>,
    digits: Vec<String>
}

fn sort_string(unsorted : &str) -> String {
    let s_slice: &str = &unsorted[..];

    let mut chars: Vec<char> = s_slice.chars().collect();
    chars.sort_by(|a, b| a.cmp(b));

    let s = String::from_iter(chars);
    s
}

impl Line {
    fn new(s : &str) -> Line {
        let mut line = s.split(" | ");
        let wires = line.next().unwrap().split(" ");
        let digits = line.next().unwrap().split(" ");
        let mut char_sorted_wires : Vec<String> = wires.map(|x| sort_string(x)).collect();
        char_sorted_wires.sort_by(|a, b| a.len().cmp(&b.len()));
        Line {
            wires: char_sorted_wires,
            digits: digits.map(|x| sort_string(x)).collect()
        }
    }
}

// 2 letters -> 1
// 3 letters -> 7
// 4 letters -> 4
// 5 letters -> 2, 3, 5
//   1-pattern in there -> 3
//   
// 6 letters -> 0, 6, 9
//   3-pattern in there -> 9
//   5-pattern in there -> 6
//   else -> 0
// 7 letters -> 8

fn solve_part_1(v : &Vec<Line>) -> usize {
    let mut num_unique_digits = 0;

    for line in v {
        for digit in &line.digits {
            match digit.len() {
                2 => num_unique_digits += 1,
                3 => num_unique_digits += 1,
                4 => num_unique_digits += 1,
                7 => num_unique_digits += 1,
                _ => ()
            }
        }
    }
    println!("{}", num_unique_digits);
    num_unique_digits
}

fn solve_part_2(v : &Vec<Line>) -> usize {
    let mut grand_total = 0;

    let mut digits = HashMap::new();
    for line in v {
        let one = &line.wires[0];
        let mut three = String::from("");
        let mut five = String::from("");
        let mut four_xor_one = String::from("");

        for w in &line.wires {
            match w.len() {
                2 => digits.insert(w, 1),
                3 => digits.insert(w, 7),
                4 => {
                    digits.insert(w, 4);
                    four_xor_one = w.clone();
                    for c in one.chars() {
                        four_xor_one.remove(four_xor_one.find(c).unwrap());
                    }
                    None
                }
                5 => {
                    if one.chars().all(|c| w.contains(c)) {
                        digits.insert(w, 3);
                        three = w.clone();
                    }
                    else if four_xor_one.chars().all(|c| w.contains(c)) {
                        digits.insert(w, 5);
                        five = w.clone();
                    }
                    else {
                        digits.insert(w, 2);
                    }
                    None
                },
                6 => {
                    if three.chars().all(|c| w.contains(c)) {
                        digits.insert(w, 9);
                    }
                    else if five.chars().all(|c| w.contains(c)) {
                        digits.insert(w, 6);
                    }
                    else {
                        digits.insert(w, 0);
                    }
                    None
                },
                7 => digits.insert(w, 8),
                _ => None
            };
        }
        let mut total = 0;
        let mut idx = 4;
        for digit in &line.digits {
            idx -= 1;
            total += digits[&digit] * 10usize.pow(idx);
        }
        grand_total += total;
    }
    println!("{}", grand_total);
    grand_total
}

fn do_solve() -> (usize, usize) {
    let filename = "data/day08/input.txt";

    let vec = load_data(filename, Line::new);

    let result1 = solve_part_1(&vec);
    let result2 = solve_part_2(&vec);
    (result1, result2)
}

fn main() {
    do_solve();
}

#[test]
fn test_day08_01() {
    let (result1, result2) = do_solve();
    assert_eq!(result1, 440);
    assert_eq!(result2, 1046281);
}
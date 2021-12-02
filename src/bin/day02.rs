use std::vec::Vec;
use aoc21::load_data;
// use std::ops;
use std::iter::Sum;


struct Command {
    command: String,
    amount: i32,
}

struct Move {
    horizontal : i32,
    vertical : i32,
}

impl Sum for Move {
    fn sum<I>(iter: I) -> Move
    where
     I: Iterator<Item = Move>
    {
        iter.fold(Move { horizontal: 0, vertical: 0 }, |a, b| Move {
            horizontal: a.horizontal + b.horizontal,
            vertical: a.vertical + b.vertical,
        })
    }
}

impl<'a> Sum<&'a Self> for Move {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Move { horizontal: 0, vertical: 0 }, |a, b| Move {
            horizontal: a.horizontal + b.horizontal,
            vertical: a.vertical + b.vertical,
        })
    }
}

impl Command {
    fn new(s : &str) -> Command {
        let mut raw_move = s.split(" ");
        let direction = String::from(raw_move.next().unwrap());
        let amount : i32 = raw_move.next().unwrap().parse().unwrap();
        Command {
            command: direction,
            amount: amount
        }
    }
}

impl Move {
    fn new(c : &Command) -> Move {
        let mut struct_move : Move = Move { horizontal: 0, vertical: 0};
        match c.command.as_str() {
            "forward" => struct_move.horizontal = c.amount,
            "down" => struct_move.vertical = c.amount,
            "up" => {
                struct_move.vertical = c.amount;
                struct_move.vertical *= -1;
            }
            &_ => panic!("Reached unreachable code")
        }
        struct_move
    }
}

// impl ops::Add<Move> for Move {
//     type Output = Move;

//     fn add(self, _rhs: Move) -> Move {
//         println!("> Foo.add(Bar) was called");
//         self.horizontal += _rhs.horizontal;
//         self.vertical += _rhs.vertical;
//         self
//     }
// }

fn solve_part_1(vec : &Vec<Command>) {
    let total : Move = vec.into_iter().map(|c| Move::new(c)).sum();
    println!("Horizontal {}, Vertical {}, Product {}",
        total.horizontal,
        total.vertical,
        total.vertical * total.horizontal);
}

fn solve_part_2(vec : &Vec<Command>) {
    let mut aim : i32 = 0;
    let mut postition : i32 = 0;
    let mut depth : i32 = 0;
    for c in vec {
        match c.command.as_str() {
            "forward" => { postition += c.amount; depth += aim * c.amount; },
            "down" => aim += c.amount,
            "up" => aim -= c.amount,
            &_ => panic!("Reached unreachable code")
        }
    }
    println!("Horizontal postition {}, Depth {}, Product {}",
        postition,
        depth,
        postition * depth);
}

fn main() {
    let filename = "data/day02/input.txt";

    let vec = load_data(filename, Command::new);
    solve_part_1(&vec);
    solve_part_2(&vec);
}
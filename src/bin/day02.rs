use std::vec::Vec;
use aoc21::load_data;
// use std::ops;
use std::iter::Sum;


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

impl Move {
    fn new(s : &str) -> Move {
        let mut raw_move = s.split(" ");
        let direction = raw_move.next().unwrap();
        let mut struct_move : Move = Move { horizontal: 0, vertical: 0};
        match direction {
            "forward" => struct_move.horizontal = raw_move.next().expect("Split failed").parse().expect("Cast failed"),
            "down" => struct_move.vertical = raw_move.next().unwrap().parse().unwrap(),
            "up" => {
                struct_move.vertical = raw_move.next().unwrap().parse().unwrap();
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

fn solve_part_1(vec : &Vec<Move>) {
    let total : Move = vec.iter().sum();
    println!("Horizontal {}, Vertical {}, Product {}",
        total.horizontal,
        total.vertical,
        total.vertical * total.horizontal);
}

fn solve_part_2(vec : &Vec<Move>) {

}

fn main() {
    let filename = "data/day02/input.txt";

    let vec = load_data(filename, Move::new);
    solve_part_1(&vec);
    solve_part_2(&vec);
}
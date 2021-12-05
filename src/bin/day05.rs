use std::vec::Vec;
use aoc21::load_data;

struct Line {
    start: (usize, usize),
    end: (usize, usize)
}

impl Line {
    fn new(s : &str) -> Line {
        let mut points = s.split(" -> ");
        let mut start = points.next().unwrap().split(",");
        let mut end = points.next().unwrap().split(",");
        Line {
            start: (start.next().unwrap().parse().unwrap(), start.next().unwrap().parse().unwrap()),
            end:   (end.next().unwrap().parse().unwrap(), end.next().unwrap().parse().unwrap()),
        }
    }
}

fn solve_part_1(vec : &Vec<Line>) {
    let mut field : Vec<Vec<usize>> = vec![vec![0; 1000]; 1000];
    let mut num_fields_greater_one = 0;
    for line in vec.into_iter().filter(|l| l.start.0 == l.end.0 || l.start.1 == l.end.1) {
        //println!("({},{}) -> ({},{})",
        //    line.start.0, line.start.1, line.end.0, line.end.1);
        if line.start.0 == line.end.0 {
            let (start_y, end_y) = if line.start.1 > line.end.1 { (line.end.1, line.start.1) } else { (line.start.1, line.end.1) };
            for y in start_y..end_y+1 {
                if field[line.start.0][y] == 1 {
                    num_fields_greater_one += 1;
                }
                field[line.start.0][y] += 1;
            }
        }
        else if line.start.1 == line.end.1 {
            let (start_x, end_x) = if line.start.0 > line.end.0 { (line.end.0, line.start.0) } else { (line.start.0, line.end.0) };
            for x in start_x..end_x+1 {
                if field[x][line.start.1] == 1 {
                    num_fields_greater_one += 1;
                }
                field[x][line.start.1] += 1;
            }
        }
    }
    // for y in 0..10 {
    //     for x in 0..10 {
    //         print!("{}", field[x][y]);
    //     }
    //     print!("\n");
    // }
    // println!("");
    println!("{}", num_fields_greater_one);
}

fn solve_part_2(vec : &Vec<Line>) {
    let mut field : Vec<Vec<usize>> = vec![vec![0; 1000]; 1000];
    let mut num_fields_greater_one = 0;
    for line in vec {
        //println!("({},{}) -> ({},{})",
        //    line.start.0, line.start.1, line.end.0, line.end.1);
        if line.start.0 == line.end.0 && line.start.1 != line.end.1 {
            let (start_y, end_y) = if line.start.1 > line.end.1 { (line.end.1, line.start.1) } else { (line.start.1, line.end.1) };
            for y in start_y..end_y+1 {
                if field[line.start.0][y] == 1 {
                    num_fields_greater_one += 1;
                }
                field[line.start.0][y] += 1;
            }
        }
        else if line.start.1 == line.end.1 && line.start.0 != line.end.0 {
            let (start_x, end_x) = if line.start.0 > line.end.0 { (line.end.0, line.start.0) } else { (line.start.0, line.end.0) };
            for x in start_x..end_x+1 {
                if field[x][line.start.1] == 1 {
                    num_fields_greater_one += 1;
                }
                field[x][line.start.1] += 1;
            }
        }
        else {//if (line.start.1-line.end.1).abs() == (line.start.0-line.end.0).abs() {
            let pos_y = line.start.1 < line.end.1;
            let pos_x = line.start.0 < line.end.0;
            let mut x = line.start.0;
            let mut y = line.start.1;
            while x != line.end.0 && y != line.end.1 {
                if field[x][y] == 1 {
                    num_fields_greater_one += 1;
                }
                field[x][y] += 1;
                x = if pos_x { x+1 } else { x-1 };
                y = if pos_y { y+1 } else { y-1 };
            }
            if x == line.end.0 && y == line.end.1 {
                if field[x][y] == 1 {
                    num_fields_greater_one += 1;
                }
                field[x][y] += 1;
            }
        }
    }
    println!("{}", num_fields_greater_one);
}

fn main() {
    let filename = "data/day05/input.txt";

    let vec = load_data(filename, Line::new);
    solve_part_1(&vec);
    solve_part_2(&vec);
}
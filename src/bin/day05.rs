use std::vec::Vec;
use aoc21::load_data;

#[derive(Copy, Clone)]
struct Point {
    x: usize,
    y: usize
}

#[derive(Copy, Clone)]
struct Line {
    start: Point,
    end: Point,
    cur: Point,
    //next: Point,
}

impl Iterator for Line {
    // We can refer to this type using Self::Item
    type Item = Point;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.start.x == self.end.x {
            if self.start.y <= self.end.y && self.cur.y <= self.end.y {
                let cur = self.cur;
                self.cur.y += 1;
                Some(cur)
            }
            else if self.start.y > self.end.y && self.cur.y >= self.end.y {
                let cur = self.cur;
                self.cur.y -= 1;
                Some(cur) 
            }
            else {
                None
            }
        }
        else if self.start.y == self.end.y {
            if self.start.x < self.end.x && self.cur.x <= self.end.x {
                let cur = self.cur;
                self.cur.x += 1;
                Some(cur)
            }
            else if self.start.x > self.end.x && self.cur.x >= self.end.x {
                let cur = self.cur;
                self.cur.x -= 1;
                Some(cur) 
            }
            else {
                None
            }
        }
        else {
            if self.start.x < self.end.x && self.cur.x <= self.end.x {
                let cur = self.cur;
                self.cur.x += 1;
                if self.start.y < self.end.y {
                    self.cur.y += 1;
                }
                else {
                    self.cur.y -= 1;
                }
                Some(cur)
            }
            else if self.start.x > self.end.x && self.cur.x >= self.end.x {
                let cur = self.cur;
                self.cur.x -= 1;
                if self.start.y < self.end.y {
                    self.cur.y += 1;
                }
                else {
                    self.cur.y -= 1;
                }
                Some(cur)
            }
            else {
                None
            }
        }
    }
}

impl Line {
    fn new(s : &str) -> Line {
        let mut points = s.split(" -> ");
        let mut start = points.next().unwrap().split(",");
        let mut end = points.next().unwrap().split(",");
        let start_point = Point{x: start.next().unwrap().parse().unwrap(), y: start.next().unwrap().parse().unwrap() };
        let end_point = Point {x: end.next().unwrap().parse().unwrap(), y: end.next().unwrap().parse().unwrap() };
        Line {
            start: start_point,
            end:   end_point,
            cur:   start_point
        }
    }
}

fn solve_part_1(vec : Vec<Line>) {
    let mut field : Vec<Vec<usize>> = vec![vec![0; 1000]; 1000];
    let mut num_fields_greater_one = 0;
    for line in vec.into_iter().filter(|l| l.start.x == l.end.x || l.start.y == l.end.y) {
        for point in line {
            field[point.x][point.y] += 1;
        }
    }
    for row in field {
        let count = row.iter().filter(|&point| point > &1).count();
        num_fields_greater_one += count;
    }
    println!("{}", num_fields_greater_one);
}

fn solve_part_2(vec : Vec<Line>) {
    let mut field : Vec<Vec<usize>> = vec![vec![0; 1000]; 1000];
    let mut num_fields_greater_one = 0;
    for line in vec {
        for point in line.into_iter() {
            field[point.x][point.y] += 1;
        }
    }
    for row in field {
        let count = row.iter().filter(|&point| point > &1).count();
        num_fields_greater_one += count;
    }
    println!("{}", num_fields_greater_one);
}

fn main() {
    let filename = "data/day05/input.txt";

    let vec = load_data(filename, Line::new);
    solve_part_1(vec.clone());
    solve_part_2(vec.clone());
}
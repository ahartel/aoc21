use std::vec::Vec;
use std::fs;


pub fn load_data(filename: &str) -> Vec<Vec<u32>> {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let entries = contents.split("\n");
    let mut vec = Vec::new();
    const RADIX: u32 = 10;
    for s in entries {
        if s != "" {
            vec.push(s.chars().map(|c| c.to_digit(RADIX).unwrap()).collect());
        }
    }
    vec
}

fn find_minima(v : &Vec<Vec<u32>>) -> Vec<(usize, usize, u32)> {
    let mut minima = Vec::new();

    for (y, row) in v.iter().enumerate() {
        for (x, cur) in row.iter().enumerate() {
            if y == 0 {
                if x == 0 {
                    if cur < &row[x+1] && cur < &v[y+1][x] {
                        minima.push((x, y, cur.clone()));
                    }
                }
                else if x+1 == row.len() {
                    if cur < &row[x-1] && cur < &v[y+1][x] {
                        minima.push((x, y, cur.clone()));
                    }
                }
                else {
                    if cur < &row[x+1] && cur < &v[y+1][x] && cur < &row[x-1] {
                        minima.push((x, y, cur.clone()));
                    }
                }
            }
            else if y+1 == v.len() {
                if x == 0 {
                    if cur < &row[x+1] && cur < &v[y-1][x] {
                        minima.push((x, y, cur.clone()));
                    }
                }
                else if x+1 == row.len() {
                    if cur < &row[x-1] && cur < &v[y-1][x] {
                        minima.push((x, y, cur.clone()));
                    }
                }
                else {
                    if cur < &row[x+1] && cur < &v[y-1][x] && cur < &row[x-1] {
                        minima.push((x, y, cur.clone()));
                    }
                }
            }
            else {
                if x == 0 {
                    if cur < &row[x+1] && cur < &v[y-1][x] && cur < &v[y+1][x] {
                        minima.push((x, y, cur.clone()));
                    }
                }
                else if x+1 == row.len() {
                    if cur < &row[x-1] && cur < &v[y-1][x] && cur < &v[y+1][x] {
                        minima.push((x, y, cur.clone()));
                    }
                }
                else {
                    if cur < &row[x+1] && cur < &v[y-1][x] && cur < &row[x-1] && cur < &v[y+1][x] {
                        minima.push((x, y, cur.clone()));
                    }
                }
            } 
        }
    }
    minima
}

fn solve_part_1(v : &Vec<Vec<u32>>) -> u32 {
    let minima = find_minima(v);
    // println!("{:?}", minima);
    let result = minima.iter().map(|x| (x.2)+1).sum();
    println!("Result part 1: {}", result);
    result
}

fn floodfill(x: usize, y: usize, basin_size: &mut usize, v: &mut Vec<Vec<u32>>) {
    if v[y][x] < 9 {
        *basin_size += 1;
        v[y][x] = 9;
        if y > 0 {
            floodfill(x, y-1, basin_size, v);
        }
        if y < v.len()-1 {
            floodfill(x, y+1, basin_size, v);
        }
        if x > 0 {
            floodfill(x-1, y, basin_size, v);
        }
        if x < v[0].len()-1 {
            floodfill(x+1, y, basin_size, v);
        }
    }
}

fn solve_part_2(v : &Vec<Vec<u32>>) -> usize {
    let minima = find_minima(v);
    let mut basins : Vec<usize> = Vec::new();
    // println!("{:?}", minima);
    for min in &minima {
        let (x, y, _) = min;
        let mut basin_size = 0;
        let mut v_temp = v.clone();
        floodfill(*x, *y, &mut basin_size, &mut v_temp);
        basins.push(basin_size);
    }
    basins.sort_by(|a, b| b.cmp(a));
    basins.drain(3..basins.len());
    let result = basins[0] * basins[1] * basins[2];
    println!("Result part 2: {}", result);
    result
}

fn do_solve() -> (u32, usize) {
    let filename = "data/day09/input.txt";

    let vec = load_data(filename);

    let result1 = solve_part_1(&vec);
    let result2 = solve_part_2(&vec);
    (result1, result2)
}

fn main() {
    do_solve();
}

#[test]
fn test_day09_01() {
    let (result1, result2) = do_solve();
    assert_eq!(result1, 502);
    assert_eq!(result2, 1330560);
}
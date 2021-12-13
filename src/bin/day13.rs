
struct Point {
    x: usize,
    y: usize
}

fn main() {
    let mut points : Vec<Point> = Vec::new();
    let mut max_x : usize = 0;
    let mut max_y : usize = 0;
    let mut lines = include_str!("../../data/day13/input.txt").lines();
    for line in &mut lines {
        if line == "" {
            break;
        }
        let mut coords = line.split(",");
        let x = coords.next().unwrap().parse::<usize>().unwrap();
        let y = coords.next().unwrap().parse::<usize>().unwrap();
        points.push(Point{x: x, y: y});
        if x > max_x {
            max_x = x;
        }
        if y > max_y {
            max_y = y;
        }
    }
    let mut fold_instruction = lines.next().unwrap().split("=");
    let is_x_fold = fold_instruction.next().unwrap().contains("x");
    let fold_col_or_line = fold_instruction.next().unwrap().parse::<usize>().unwrap();
    println!("{}, {}", is_x_fold, fold_col_or_line);
    let mut matrix : Vec<Vec<usize>> = vec![vec![0; max_y+1]; max_x+1];
    for point in points {
        if is_x_fold && point.x > fold_col_or_line {
            matrix[point.y][fold_col_or_line-(point.x-fold_col_or_line)] += 1;
        }
        else {
            if !is_x_fold && point.y > fold_col_or_line {
                matrix[fold_col_or_line-(point.y-fold_col_or_line)][point.x] += 1;
            }
            else {
                matrix[point.y][point.x] += 1;
            }
        }
    }
    let mut total_points_after_fold = 0;
    for row in matrix {
        for col in row {
            if col > 0 {
                total_points_after_fold += 1;
            }
        }
    }
    println!("{}", total_points_after_fold);
}
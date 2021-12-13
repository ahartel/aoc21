
struct Point {
    x: usize,
    y: usize
}

fn print_matrix(matrix: &Vec<Vec<char>>, max_x: usize, max_y: usize) {
    // print matrix
    for row in 0..max_y+1 {
        for col in 0..max_x+1 {
            print!("{}", matrix[row][col]);
        }
        print!("\n");
    }
    print!("\n");
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
    let mut matrix : Vec<Vec<char>> = vec![vec!['.'; max_x+1]; max_y+1];
    for point in points {
        matrix[point.y][point.x] = '#';
    }

    let mut instruction = lines.next();
    while instruction != None {
        let mut fold_instruction = instruction.unwrap().split("=");
        let is_x_fold = fold_instruction.next().unwrap().contains("x");
        let fold_col_or_line = fold_instruction.next().unwrap().parse::<usize>().unwrap();
        println!("Fold instruction: {}, {}", is_x_fold, fold_col_or_line);

        for y in 0..(max_y+1) {
            for x in 0..(max_x+1) {
                if is_x_fold && x > fold_col_or_line && matrix[y][x] == '#' {
                    matrix[y][fold_col_or_line-(x-fold_col_or_line)] = '#';
                }
                else {
                    if !is_x_fold && y > fold_col_or_line && matrix[y][x] == '#' {
                        matrix[fold_col_or_line-(y-fold_col_or_line)][x] = '#';
                    }
                }
            }
        }

        // prepare for next iteration
        instruction = lines.next();
        if is_x_fold {
            max_x = fold_col_or_line-1; 
        }
        else {
            max_y = fold_col_or_line-1;
        }

        // count number of points in matrix
        let mut total_points_after_fold = 0;
        for row in 0..(max_y+1) {
            for col in 0..(max_x+1) {
                if matrix[row][col] == '#' {
                    total_points_after_fold += 1;
                }
            }
        }
        println!("Number of #'s in the matrix is {}", total_points_after_fold);
    }
    print_matrix(&matrix, max_x, max_y);
}
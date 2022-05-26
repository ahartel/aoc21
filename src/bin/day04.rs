use std::fs;
use std::fmt;

#[derive(Clone)]
struct Board {
    rows: Vec<(Vec<String>, Vec<bool>)>
}

impl Board {
    fn mark_number(self: &mut Self, number: &str) {
        for row in &mut self.rows {
            for idx in 0..row.0.len() {
                if row.0[idx] == number {
                    row.1[idx] = true;
                }
            }
        }
    }

    fn get_sum_of_unmarked_numbers(self: &Self) -> usize {
        let mut sum : usize = 0;
        for row in &self.rows {
            for idx in 0..row.0.len() {
                if row.1[idx] == false {
                    let number: usize = row.0[idx].parse().unwrap();
                    sum += number;
                }
            }
        }
        sum
    }

    fn has_complete_row_or_col(self: &Self) -> bool {
        let mut found = false;
        for row in &self.rows {
            let mut row_is_set = true;
            for set in &row.1 {
                row_is_set &= set;
            }
            found |= row_is_set;
        }
        if !found {
            for idx in 0..self.rows[0].0.len() {
                let mut col_is_set = true;
                for row in &self.rows {
                    col_is_set &= row.1[idx];
                }
                found |= col_is_set;
            }
        }
        found
    }
}

impl fmt::Display for Board {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        for row in &self.rows {
            let _var_name = write!(f, "{:?}\n", row.0);
            let _var_name = write!(f, "{:?}\n", row.1);
        }
        write!(f, "\n")
    }
}

fn load_bingo(filename: &String) -> (String, Vec<Board>) {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let mut lines = contents.split("\n");
    let numbers = lines.next().unwrap();
    println!("{}", numbers);
    lines.next();
    let mut boards : Vec<Board> = Vec::new();
    let mut current_rows : Vec<(Vec<String>, Vec<bool>)> = Vec::new();
    for line in lines {
        if line == "" {
            boards.push(Board{rows: current_rows.clone()});
            current_rows.clear();
        }
        else {
            let numbers : Vec<String> = line.split(" ").filter(|x| *x != "").map(|x| String::from(x)).collect();
            let vector_size = numbers.len();
            current_rows.push((numbers, vec![false; vector_size]));
        }
    }
    boards.push(Board{rows: current_rows.clone()});

    (numbers.to_string(), boards)
}

fn solve_part_1(numbers: &String, mut boards: Vec<Board>) {
    for number in numbers.split(",") {
        //println!("{}", number);
        for board in &mut boards {
            board.mark_number(&number);
            //println!("{}", board);
            if board.has_complete_row_or_col() {
                let sum_of_unmarked = board.get_sum_of_unmarked_numbers();
                let last_number : usize = number.parse().unwrap();
                println!("sum: {}, last: {}, product: {}",
                    sum_of_unmarked,
                    last_number,
                    sum_of_unmarked * last_number);
                return;
            }
        }
    }
}

fn solve_part_2(numbers: &String, mut boards: Vec<Board>) {
    let mut last_completed_board = 0;
    let mut number_that_completed_last : usize = 0;
    for number in numbers.split(",") {
        //println!("{}", number);
        for idx in 0..boards.len() {
            let board = &mut boards[idx];
            if !board.has_complete_row_or_col() {
                board.mark_number(&number);
                //println!("{}", board);
                if board.has_complete_row_or_col() {
                    last_completed_board = idx;
                    number_that_completed_last = number.parse().unwrap();
                }
            }
        }
    }
    let sum_of_unmarked = boards[last_completed_board].get_sum_of_unmarked_numbers();
    let last_number : usize = number_that_completed_last;
    println!("sum: {}, last: {}, product: {}",
        sum_of_unmarked,
        last_number,
        sum_of_unmarked * last_number);
}

fn main() {
    let filename = String::from("data/day04/input.txt");
    let (numbers, vec) = load_bingo(&filename);
    solve_part_1(&numbers, vec.clone());
    solve_part_2(&numbers, vec.clone());
}
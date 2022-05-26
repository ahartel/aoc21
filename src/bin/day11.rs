fn increment_neighbours(state: &mut Vec<Vec<u32>>, row: usize, col: usize) -> bool {
    let mut bool_found_above_nine = false;
    if row > 0 {
        if col > 0 {
            if state[row - 1][col - 1] > 0 {
                state[row - 1][col - 1] += 1;
                bool_found_above_nine |= state[row - 1][col - 1] > 9;
            }
        }
        if state[row - 1][col] > 0 {
            state[row - 1][col] += 1;
            bool_found_above_nine |= state[row - 1][col] > 9;
        }
        if col < state[row - 1].len() - 1 {
            if state[row - 1][col + 1] > 0 {
                state[row - 1][col + 1] += 1;
                bool_found_above_nine |= state[row - 1][col + 1] > 9;
            }
        }
    }
    if col > 0 {
        if state[row][col - 1] > 0 {
            state[row][col - 1] += 1;
            bool_found_above_nine |= state[row][col - 1] > 9;
        }
    }
    if col < state[row].len() - 1 {
        if state[row][col + 1] > 0 {
            state[row][col + 1] += 1;
            bool_found_above_nine |= state[row][col + 1] > 9;
        }
    }
    if row < state.len() - 1 {
        if col > 0 {
            if state[row + 1][col - 1] > 0 {
                state[row + 1][col - 1] += 1;
                bool_found_above_nine |= state[row + 1][col - 1] > 9;
            }
        }
        if state[row + 1][col] > 0 {
            state[row + 1][col] += 1;
            bool_found_above_nine |= state[row + 1][col] > 9;
        }
        if col < state[row + 1].len() - 1 {
            if state[row + 1][col + 1] > 0 {
                state[row + 1][col + 1] += 1;
                bool_found_above_nine |= state[row + 1][col + 1] > 9;
            }
        }
    }
    bool_found_above_nine
}

fn iterate(state: &Vec<Vec<u32>>) -> (usize, Vec<Vec<u32>>) {
    let mut num_flashes = 0;
    let mut new_state: Vec<Vec<u32>> = state.clone();
    for row in &mut new_state {
        for col in &mut row.into_iter() {
            *col += 1;
        }
    }
    let mut do_continue: bool = true;
    while do_continue {
        do_continue = false;
        for row in 0..new_state.len() {
            for col in 0..new_state[0].len() {
                let cur = &mut new_state[row][col];
                if *cur > 9 {
                    *cur = 0;
                    num_flashes += 1;
                    do_continue |= increment_neighbours(&mut new_state, row, col);
                }
            }
        }
    }
    (num_flashes, new_state)
}

fn solve_part_1(mut state: Vec<Vec<u32>>) -> usize {
    let mut total_flashes = 0;
    let mut step_flashes;
    for _ in 0..100 {
        let res = iterate(&state);
        step_flashes = res.0;
        state = res.1;
        total_flashes += step_flashes;
        //println!("Run {}: {} flashes", i, step_flashes);
        //printstate(&initial_state);
    }
    println!("Total flashes: {}", total_flashes);
    total_flashes
}

fn solve_part_2(mut state: Vec<Vec<u32>>) -> usize {
    let mut step_flashes = 0;
    let mut num_steps = 0;
    while step_flashes < 100 {
        num_steps += 1;
        let res = iterate(&state);
        step_flashes = res.0;
        state = res.1;
        //println!("Run {}: {} flashes", i, step_flashes);
        //printstate(&initial_state);
    }
    println!("Number of steps: {}", num_steps);
    num_steps
}

fn main() {
    const RADIX: u32 = 10;

    let initial_state: Vec<Vec<u32>> = include_str!("../../data/day11/input.txt")
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(RADIX).unwrap()).collect())
        .collect();

    solve_part_1(initial_state.clone());
    solve_part_2(initial_state.clone());
}

#[test]
fn test_day11_01() {
    const RADIX: u32 = 10;

    let initial_state: Vec<Vec<u32>> = include_str!("../../data/day11/input.txt")
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(RADIX).unwrap()).collect())
        .collect();

    assert_eq!(solve_part_1(initial_state.clone()), 1585);
    assert_eq!(solve_part_2(initial_state.clone()), 382);
}

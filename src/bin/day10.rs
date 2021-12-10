


fn compute_closing_sequence(line: &str, idx: usize, closing: &mut Vec<char>) -> Result<(char, usize), (char, usize)> {
    let character = line.chars().nth(idx);
    match character {
        Some('{') => {
            let res = compute_closing_sequence(&line, idx+1, closing);
            match res {
                Err(_) => res,
                Ok(char_pos) => {
                    match char_pos.0 {
                        '}' => compute_closing_sequence(&line, char_pos.1 + 1, closing),
                        'e' => {
                            closing.push('}');
                            res
                        },
                        _ => Err((char_pos.0, char_pos.1))
                    }
                }
            }
        },
        Some('[') => {
            let res = compute_closing_sequence(&line, idx+1, closing);
            match res {
                Err(_) => res,
                Ok(char_pos) => {
                    match char_pos.0 {
                        ']' => compute_closing_sequence(&line, char_pos.1 + 1, closing),
                        'e' => {
                            closing.push(']');
                            res
                        },
                        _ => Err((char_pos.0, char_pos.1))
                    }
                }
            }
        },
        Some('(') => {
            let res = compute_closing_sequence(&line, idx+1, closing);
            match res {
                Err(_) => res,
                Ok(char_pos) => {
                    match char_pos.0 {
                        ')' => compute_closing_sequence(&line, char_pos.1 + 1, closing),
                        'e' => {
                            closing.push(')');
                            res
                        },
                        _ => Err((char_pos.0, char_pos.1))
                    }
                }
            }
        },
        Some('<') => {
            let res = compute_closing_sequence(&line, idx+1, closing);
            match res {
                Err(_) => res,
                Ok(char_pos) => {
                    match char_pos.0 {
                        '>' => compute_closing_sequence(&line, char_pos.1 + 1, closing),
                        'e' => {
                            closing.push('>');
                            res
                        },
                        _ => Err((char_pos.0, char_pos.1))
                    }
                }
            }
        },
        Some(c) => Ok((c, idx)),
        None => Ok(('e', idx)),
        _ => Err((' ', idx))
    }
}


fn get_first_illegal_character(line: &str, idx: usize) -> Result<(char, usize), (char, usize)> {
    let character = line.chars().nth(idx);
    match character {
        Some('{') => {
            let res = get_first_illegal_character(&line, idx+1);
            match res {
                Err(_) => res,
                Ok(char_pos) => {
                    match char_pos.0 {
                        '}' => get_first_illegal_character(&line, char_pos.1 + 1),
                        'e' => res,
                        _ => Err((char_pos.0, char_pos.1))
                    }
                }
            }
        },
        Some('[') => {
            let res = get_first_illegal_character(&line, idx+1);
            match res {
                Err(_) => res,
                Ok(char_pos) => {
                    match char_pos.0 {
                        ']' => get_first_illegal_character(&line, char_pos.1 + 1),
                        'e' => res,
                        _ => Err((char_pos.0, char_pos.1))
                    }
                }
            }
        },
        Some('(') => {
            let res = get_first_illegal_character(&line, idx+1);
            match res {
                Err(_) => res,
                Ok(char_pos) => {
                    match char_pos.0 {
                        ')' => get_first_illegal_character(&line, char_pos.1 + 1),
                        'e' => res,
                        _ => Err((char_pos.0, char_pos.1))
                    }
                }
            }
        },
        Some('<') => {
            let res = get_first_illegal_character(&line, idx+1);
            match res {
                Err(_) => res,
                Ok(char_pos) => {
                    match char_pos.0 {
                        '>' => get_first_illegal_character(&line, char_pos.1 + 1),
                        'e' => res,
                        _ => Err((char_pos.0, char_pos.1))
                    }
                }
            }
        },
        Some(c) => Ok((c, idx)),
        None => Ok(('e', idx)),
        _ => Err((' ', idx))
    }
}

fn convert_failing_char_to_score(res: Result<(char, usize), (char, usize)>) -> usize {
    //println!("{:?}", res);
    match res {
        Ok(_) => 0,
        Err((c, i)) => {
            match &c {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                &_  => 0
            }
        }
    }
}

fn convert_sequence_to_score(v: Vec<char>) -> usize {
    // println!("{:?}", v);
    let mut total = 0;
    for c in v {
        match c {
            ')' => total = total * 5 + 1,
            ']' => total = total * 5 + 2,
            '}' => total = total * 5 + 3,
            '>' => total = total * 5 + 4,
            _ => panic!("unexpected character")
        }
    }

    total
}

fn main() {
    let score1 : usize = include_str!("../../data/day10/input.txt")
        .lines()
        .map(|l| get_first_illegal_character(l, 0))
        .map(|c| convert_failing_char_to_score(c))
        .sum();
    println!("{}", score1);

    let mut scores2 : Vec<usize> = include_str!("../../data/day10/input.txt")
        .lines()
        .map(|l| { let mut vec = Vec::new(); compute_closing_sequence(l, 0, &mut vec); vec })
        .map(|v| convert_sequence_to_score(v))
        .filter(|s| s > &0)
        .collect();
    scores2.sort();
    println!("{}, {}", scores2[scores2.len()/2], scores2.len()/2);
}
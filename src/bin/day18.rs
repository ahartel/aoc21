

fn explode_one(number: &String) -> (String, bool) {
    // 1. find exploding pair
    let mut depth = 0;
    let mut str_idx = 0;
    let mut last_comma_idx = usize::MAX;
    let mut last_open_idx = usize::MAX;
    let mut is_regular_pair = false;
    let mut last_regular_before_explode_idx = (false, usize::MAX);
    let mut first_regular_after_explode_idx = (false, usize::MAX);
    let mut start_of_explode = usize::MAX;
    let mut end_of_explode = usize::MAX;
    for c in number.chars() {
        match c {
            '[' => {
                last_open_idx = str_idx;
                depth += 1;
                is_regular_pair = true;
            },
            ']' => {
                if is_regular_pair && depth > 4 && start_of_explode == usize::MAX {
                    start_of_explode = last_open_idx;
                    end_of_explode = str_idx;
                    //break;
                }
                depth -= 1;
                is_regular_pair = false;
            },
            ',' => {
                last_comma_idx = str_idx;
            }
            '0'..='9' => {
                if start_of_explode == usize::MAX && depth <= 4 {
                    if last_open_idx < usize::MAX && (last_open_idx > last_comma_idx || last_comma_idx == usize::MAX) {
                        last_regular_before_explode_idx = (true, last_open_idx + 1);
                    }
                    else if last_comma_idx < usize::MAX {
                        last_regular_before_explode_idx = (false, last_comma_idx + 1);
                    }
                }
                else if start_of_explode < usize::MAX && first_regular_after_explode_idx.1 == usize::MAX {
                    if last_open_idx > last_comma_idx {
                        first_regular_after_explode_idx = (true, last_open_idx + 1);
                    }
                    else {
                        first_regular_after_explode_idx = (false, last_comma_idx + 1);
                    }
                    break;
                }
            },
            _ => {
                panic!("Unexpected character {}", c);
            }
        }
        str_idx += 1;
    }
    // Produce a new string
    let mut res = String::new();
    if start_of_explode < usize::MAX {
        // read exploding pair
        let left_splinter : usize;
        let right_splinter : usize;
        {
            let comma_idx = number[start_of_explode..].find(',').unwrap() + start_of_explode;
            let closing_idx = number[comma_idx..].find(']').unwrap() + comma_idx;
            left_splinter = number[start_of_explode+1..comma_idx].parse::<usize>().unwrap();
            right_splinter = number[comma_idx+1..closing_idx].parse::<usize>().unwrap();
        }
        // parse regular before explosion
        if last_regular_before_explode_idx.1 < usize::MAX
        {
            res.push_str(&number[0..last_regular_before_explode_idx.1]);
            let closing : usize;
            if last_regular_before_explode_idx.0 {
                closing = number[last_regular_before_explode_idx.1..].find(',').unwrap() + last_regular_before_explode_idx.1;
            }
            else {
                closing = number[last_regular_before_explode_idx.1..].find(']').unwrap() + last_regular_before_explode_idx.1;
            }
            let prev_regular = number[last_regular_before_explode_idx.1..closing].parse::<usize>().unwrap();
            res.push_str(&(prev_regular + left_splinter).to_string());
            res.push_str(&number[closing..start_of_explode]);
        }
        else {
            res.push_str(&number[0..start_of_explode]);
        }
        res.push('0');
        // parse regular after explosion
        if first_regular_after_explode_idx.1 < usize::MAX
        {
            res.push_str(&number[end_of_explode+1..first_regular_after_explode_idx.1]);
            let closing : usize;
            if first_regular_after_explode_idx.0 {
                closing = number[first_regular_after_explode_idx.1..].find(',').unwrap() + first_regular_after_explode_idx.1;
            }
            else {
                closing = number[first_regular_after_explode_idx.1..].find(']').unwrap() + first_regular_after_explode_idx.1;
            }
            let next_regular = number[first_regular_after_explode_idx.1..closing].parse::<usize>().unwrap();
            res.push_str(&(next_regular + right_splinter).to_string());
            res.push_str(&number[closing..]);
        }
        else {
            res.push_str(&number[end_of_explode+1..]);
        }
        (res, true)
    }
    else {
        res.push_str(&number);
        (res, false)
    }
}

fn split_one(number: &String) -> (String, bool) {
    let mut split_pos = usize::MAX;
    let mut last_was_digit = false;
    let mut str_idx = 0;

    for c in number.chars() {
        match c {
            '[' => {
                last_was_digit = false;
            },
            ']' => {
                last_was_digit = false;
            },
            ',' => {
                last_was_digit = false;
            }
            '0'..='9' => {
                if last_was_digit {
                    split_pos = str_idx - 1;
                    break;
                }
                last_was_digit = true;
            },
            _ => {
                panic!("Unexpected character {}", c);
            }
        }
        str_idx += 1;
    }

    let mut res = String::new();
    if split_pos < usize::MAX {
        res.push_str(&number[0..split_pos]);
        let to_split = number[split_pos..split_pos+2].parse::<usize>().unwrap();
        let new_left = ((to_split/2) as f64).floor() as usize;
        let new_right = if to_split%2 == 0 { new_left } else { new_left+1 };
        res.push('[');
        res.push_str(&new_left.to_string());
        res.push(',');
        res.push_str(&new_right.to_string());
        res.push(']');
        res.push_str(&number[split_pos+2..]);
    }
    else {
        res.push_str(&number);
    }
    (res, split_pos < usize::MAX)
}

fn add(a : &String, b: &String) -> String {
    // println!("Adding two numbers:");
    // println!(" a={}", a);
    // println!(" b={}", b);
    let mut res = String::new();
    res.push('[');
    res.push_str(a);
    res.push(',');
    res.push_str(b);
    res.push(']');
    // construct a new empty vector
    // add a root element to the vector, left child is at pos 1
    // push the elements of the first number into the new vector
    // set right child of root to vec.len()
    // push the elements of the second number into the new vector
    // start the explode/split algorithm
    loop {
        let (exploded, did_explode) = explode_one(&res);
        // println!(" After explode: {}", &exploded);
        if did_explode {
            res = exploded;
            continue;
        }
        let (split, did_split) = split_one(&res);
        // println!(" After split:   {}", &split);
        if !did_explode && !did_split {
            break;
        }
        else if did_split {
            res = split;
        }
    }
    // println!(" Added number:  {}", &res);
    res
}

fn magnitude(number: &String, idx: &mut usize) -> usize {
    let mut c = number.chars().nth(*idx).unwrap();
    let mut my_magnitude = 0;
    loop {
        *idx += 1;
        match c {
            '[' => {
                my_magnitude += 3 * magnitude(number, idx);
            },
            ']' => {
                break;
            },
            ',' => {
                my_magnitude += 2 * magnitude(number, idx);
            }
            '0'..='9' => {
                let peek = number.chars().nth(*idx).unwrap();
                match peek {
                    '0'..='9' => {
                        *idx += 1;
                        my_magnitude = number[*idx-2..*idx].parse::<usize>().unwrap();
                    }
                    _ => {
                        my_magnitude = number[*idx-1..*idx].parse::<usize>().unwrap();
                    }
                }
                break;
            },
            _ => {
                panic!("Unexpected character {}", c);
            }
        }
        c = number.chars().nth(*idx).unwrap();
    }
    my_magnitude
}

fn main() {
    let summands : Vec<String> = include_str!("../../data/day18/input.txt")
        .lines()
        .map(|s| String::from(s))
        .collect();
    // for num in &summands {
    //     println!("{}", num);
    // }
    let mut sum = summands[0].clone();
    for i in 1..summands.len() {
        sum = add(&sum, &summands[i]);
    }

    println!("{}", &sum);
    let mut idx = 0;
    println!("Magnitude of part 1: {}", magnitude(&sum, &mut idx));

    let mut max_magnitude = 0;
    for i in 0..summands.len() {
        for j in 0..summands.len() {
            if i == j {
                continue;
            }
            let mut idx = 0;
            let mag = magnitude(&add(&summands[i], &summands[j]), &mut idx);
            if mag > max_magnitude {
                max_magnitude = mag;
            }
        }
    }
    println!("Magnitude of part 2: {}", max_magnitude);
}

#[test]
fn test_day18_explode_one() {
    {
        let mut line = String::from("[[[[[9,8],1],2],3],4]");
        let mut did_explode = false;
        let (line, did_explode) = explode_one(&line);
        assert_eq!(did_explode, true);
        assert_eq!(line, "[[[[0,9],2],3],4]");
        let (line, did_explode) = explode_one(&line);
        assert_eq!(did_explode, false);
        assert_eq!(line, "[[[[0,9],2],3],4]");
    }
    {
        let mut line = String::from("[[[[1,[9,8]],2],3],4]");
        let mut did_explode = false;
        let (line, did_explode) = explode_one(&line);
        assert_eq!(did_explode, true);
        assert_eq!(line, "[[[[10,0],10],3],4]");
        let (line, did_explode) = explode_one(&line);
        assert_eq!(did_explode, false);
        assert_eq!(line, "[[[[10,0],10],3],4]");
    }
    {
        let line = String::from("[7,[6,[5,[4,[3,2]]]]]");
        let (line, did_explode) = explode_one(&line);
        assert_eq!(did_explode, true);
        assert_eq!(line, "[7,[6,[5,[7,0]]]]");
    }
    {
        let line = String::from("[[6,[5,[4,[3,2]]]],1]");
        let (line, did_explode) = explode_one(&line);
        assert_eq!(did_explode, true);
        assert_eq!(line, "[[6,[5,[7,0]]],3]");
    }
    {
        let line = String::from("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]");
        let (line, did_explode) = explode_one(&line);
        assert_eq!(did_explode, true);
        assert_eq!(line, "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");
        let (line, did_explode) = explode_one(&line);
        assert_eq!(did_explode, true);
        assert_eq!(line, "[[3,[2,[8,0]]],[9,[5,[7,0]]]]");
    }
    {
        let line = String::from("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");
        let (line, did_explode) = explode_one(&line);
        assert_eq!(did_explode, true);
        assert_eq!(line, "[[3,[2,[8,0]]],[9,[5,[7,0]]]]");
    }
    {
        let line = String::from("[[[[0,0],[[1,1],[0,0]]],0],0]");
        let (line, did_explode) = explode_one(&line);
        assert_eq!(did_explode, true);
        assert_eq!(line, "[[[[0,1],[0,[1,0]]],0],0]");
    }
}

#[test]
fn test_day18_split_one() {
    {
        let mut line = String::from("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]");
        {
            let res = explode_one(&line);
            assert_eq!(res.1, true);
            assert_eq!(res.0, "[[[[0,7],4],[7,[[8,4],9]]],[1,1]]");
            line = res.0;
        }
        {
            let res = explode_one(&line);
            assert_eq!(res.1, true);
            assert_eq!(res.0, "[[[[0,7],4],[15,[0,13]]],[1,1]]");
            line = res.0;
        }
        {
            let res = split_one(&line);
            assert_eq!(res.1, true);
            assert_eq!(res.0, "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]");
            line = res.0;
        }
        {
            let res = split_one(&line);
            assert_eq!(res.1, true);
            assert_eq!(res.0, "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]");
            line = res.0;
        }
        {
            let res = explode_one(&line);
            assert_eq!(res.1, true);
            assert_eq!(res.0, "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
            line = res.0;
        }
    }
}
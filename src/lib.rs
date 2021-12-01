use std::fs;

pub fn load_data<T>(filename: &str, f: fn(&str) -> T) -> Vec<T> {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let entries = contents.split("\n");
    let mut vec = Vec::new();
    for s in entries {
        if s != "" {
            vec.push(f(s))
        }
    }
    vec
}

pub fn load_data_blocks<T>(filename: &str, f: fn(&Vec<&str>) -> T) -> Vec<T> {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let entries = contents.split("\n");
    let mut target_vec = Vec::new();
    let mut temp_vec = Vec::new();
    for e in entries {
        // println!("{}", e);
        if e.len() == 0 || e.chars().nth(0).unwrap() == '\r' {
            target_vec.push(f(&temp_vec));
            temp_vec.clear();
        }
        else {
            temp_vec.push(e);
        }
    }
    target_vec
}
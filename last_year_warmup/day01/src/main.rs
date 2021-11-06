use std::fs;
use std::env;
use std::vec::Vec;

fn main() {
    let filename = "input.txt";
    let path = env::current_dir().unwrap();
    println!("The current directory is {}", path.display());
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let entries = contents.split("\n");
    let mut vec = Vec::new();
    for s in entries {
        if s != "" {
            let i : u32 = s.parse().unwrap();
            vec.push(i);
            // println!("{}", i);
        }
    }
    let len = vec.len();
    for i in 0..len {
        for j in (i+1)..len {
            if vec[i]+vec[j] == 2020 {
                println!("{} * {} = {}", vec[i], vec[j], vec[i]*vec[j]);
            }
        }
    }
}

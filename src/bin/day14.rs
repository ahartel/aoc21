use std::collections::HashMap;
use std::hash::Hash;

fn insert_or_increment<T: Eq + Hash + Clone>(hashmap: &mut HashMap<T, usize>, key: &T, inc: usize) {
    if !hashmap.contains_key(key) {
        hashmap.insert(key.clone(), 0);
    }
    let count = hashmap.get_mut(key).unwrap();
    *count += 1;
    if inc > 1 {
        *count += inc - 1;
    }
}

fn main() {
    let mut insertions : HashMap<String, char> = HashMap::new();
    let mut pairs : HashMap<String, usize> = HashMap::new();
    let mut char_counts : HashMap<char, usize> = HashMap::new();

    let mut lines = include_str!("../../data/day14/input.txt").lines();
    let polymer = lines.next().unwrap().to_string();
    lines.next(); // skip newline
    for line in lines {
        let mut pair = line.split(" -> ");
        insertions.insert(pair.next().unwrap().to_string(), pair.next().unwrap().chars().nth(0).unwrap());
    }

    let mut idx : usize = 1;
    while idx < polymer.len() {
        let chars_at_pos = &polymer[idx-1..idx+1];
        let first_char = chars_at_pos.chars().nth(0).unwrap();
        insert_or_increment(&mut char_counts, &first_char, 1);
        let string_at_pos = String::from(chars_at_pos);
        insert_or_increment(&mut pairs, &string_at_pos, 1);
        idx += 1;
    }
    let last_char = polymer.chars().nth(polymer.len()-1).unwrap();
    insert_or_increment(&mut char_counts, &last_char, 1); 

    // println!("Initial pairs");
    // for (k, v) in &pairs {
    //     println!("{}: {}", k, v);
    // }
    // println!("Initial counts");
    // for (k, v) in &char_counts {
    //     println!("{}: {}", k, v);
    // }

    for _run in 0..40 {
        let mut new_pairs : HashMap<String, usize> = HashMap::new();
        for (pair, count) in &pairs {
            let insertion = insertions.get(pair).unwrap();
            insert_or_increment(&mut char_counts, &insertion, *count);
            let first_pair = format!("{}{}", pair.chars().nth(0).unwrap(), insertion);
            let second_pair = format!("{}{}", insertion, pair.chars().nth(1).unwrap());
            //println!("Run {}: {} -> {}, {}", run, pair, first_pair, second_pair);
            insert_or_increment(&mut new_pairs, &first_pair, *count);
            insert_or_increment(&mut new_pairs, &second_pair, *count);
        }
        pairs = new_pairs;
    }
    // println!("Final pairs");
    // for (k, v) in &pairs {
    //     println!("{}: {}", k, v);
    // }
    // println!("Final counts");
    // for (k, v) in &char_counts {
    //     println!("{}: {}", k, v);
    // }

    let max = char_counts.iter().max_by(|a, b| a.1.cmp(&b.1)).map(|(_k, v)| v).unwrap();
    let min = char_counts.iter().min_by(|a, b| a.1.cmp(&b.1)).map(|(_k, v)| v).unwrap();
    println!("{}, {}, {}", min, max, max-min);
}
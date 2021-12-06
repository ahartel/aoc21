fn read_data(v : &mut [usize; 9]) {
    //let fish = "3,4,3,1,2".split(",");
    let fish = "2,3,1,3,4,4,1,5,2,3,1,1,4,5,5,3,5,5,4,1,2,1,1,1,1,1,1,4,1,1,1,4,1,3,1,4,1,1,4,1,3,4,5,1,1,5,3,4,3,4,1,5,1,3,1,1,1,3,5,3,2,3,1,5,2,2,1,1,4,1,1,2,2,2,2,3,2,1,2,5,4,1,1,1,5,5,3,1,3,2,2,2,5,1,5,2,4,1,1,3,3,5,2,3,1,2,1,5,1,4,3,5,2,1,5,3,4,4,5,3,1,2,4,3,4,1,3,1,1,2,5,4,3,5,3,2,1,4,1,4,4,2,3,1,1,2,1,1,3,3,3,1,1,2,2,1,1,1,5,1,5,1,4,5,1,5,2,4,3,1,1,3,2,2,1,4,3,1,1,1,3,3,3,4,5,2,3,3,1,3,1,4,1,1,1,2,5,1,4,1,2,4,5,4,1,5,1,5,5,1,5,5,2,5,5,1,4,5,1,1,3,2,5,5,5,4,3,2,5,4,1,1,2,4,4,1,1,1,3,2,1,1,2,1,2,2,3,4,5,4,1,4,5,1,1,5,5,1,4,1,4,4,1,5,3,1,4,3,5,3,1,3,1,4,2,4,5,1,4,1,2,4,1,2,5,1,1,5,1,1,3,1,1,2,3,4,2,4,3,1".split(",");
    for f in fish {
        let idx : usize = f.parse().unwrap();
        v[idx] += 1;
    }
}

fn solve(v : &mut [usize; 9], days: usize) -> usize {
    for day in 0..days {
        let mut temp_v : [usize; 9] = [0; 9];
        //println!("{:?}", v);
        for idx in 0..v.len() {
            if idx == 0 {
                temp_v[6] += v[idx];
                temp_v[8] += v[idx];
            }
            else {
                temp_v[idx-1] += v[idx];
            }
        }
        *v = temp_v;
    }
    let solution = v.iter().sum::<usize>();
    println!("{}", solution);
    solution
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn do_solve() -> (usize, usize) {
    let mut v : [usize; 9] = [0; 9];
    //print_type_of(&v);
    read_data(&mut v);
    let result1 = solve(&mut v, 80);
    let result2 = solve(&mut v, 256-80);
    (result1, result2)
}

fn main() {
    do_solve();
}

#[test]
fn test() {
    let (result1, result2) = do_solve();
    assert_eq!(result1, 358214);
    assert_eq!(result2, 1622533344325);
}
struct TargetArea {
    x_left: isize,
    x_right: isize,
    y_top: isize,
    y_bottom: isize,
}

impl TargetArea {
    fn contains(&self, x: isize, y: isize) -> bool {
        self.x_left <= x && x <= self.x_right && self.y_bottom <= y && y <= self.y_top
    }

    fn passed(&self, x: isize, y: isize) -> bool {
        self.x_right < x || self.y_bottom > y
    }
}

fn simulate_probe(mut v_x: isize, mut v_y: isize, target: &TargetArea) -> (bool, isize) {
    let mut x = 0;
    let mut y = 0;
    let mut max_y_position = 0;
    while !target.contains(x, y) && !target.passed(x, y) {
        x += v_x;
        y += v_y;
        if y > max_y_position {
            max_y_position = y;
        }
        if v_x > 0 {
            v_x -= 1;
        } else if v_x < 0 {
            v_x += 1;
        }
        v_y -= 1;
    }
    if target.contains(x, y) {
        (true, max_y_position)
    } else {
        (false, max_y_position)
    }
}

fn main() {
    {
        let target = TargetArea {
            x_left: 20,
            x_right: 30,
            y_bottom: -10,
            y_top: -5,
        };

        assert_eq!(true, simulate_probe(7, 2, &target).0);
        assert_eq!(true, simulate_probe(6, 3, &target).0);
        assert_eq!(true, simulate_probe(9, 0, &target).0);
        assert_eq!(false, simulate_probe(17, -4, &target).0);
        assert_eq!(45, simulate_probe(6, 9, &target).1);
    }

    {
        let target = TargetArea {
            x_left: 217,
            x_right: 240,
            y_bottom: -126,
            y_top: -69,
        };
        let mut max_y = 0;
        for y in 0..1000 {
            for x in 0..1000 {
                let v_x: isize = x.try_into().unwrap();
                let v_y: isize = y.try_into().unwrap();
                let res = simulate_probe(v_x, v_y, &target);
                if res.0 && res.1 > max_y {
                    max_y = res.1;
                }
            }
        }
        println!("Max y: {}", max_y);
    }

    {
        let target = TargetArea {
            x_left: 217,
            x_right: 240,
            y_bottom: -126,
            y_top: -69,
        };
        let mut num_hits = 0;
        for y in -1000..1000 {
            for x in 0..1000 {
                let v_x: isize = x.try_into().unwrap();
                let v_y: isize = y.try_into().unwrap();
                let res = simulate_probe(v_x, v_y, &target);
                if res.0 {
                    num_hits += 1;
                }
            }
        }
        println!("Number of hits: {}", num_hits);
    }
}

fn extract_scanners(line: &str, scanners: &mut Vec<Vec<(i32, i32, i32)>>) {
    println!("{}", line);
    if line.len() > 0 {
        match &line[0..3] {
            "---" => scanners.push(Vec::new()),
            _ => {
                // split string
                let mut split = line.split(",");
                let x = split.next().unwrap().parse::<i32>().unwrap();
                let y = split.next().unwrap().parse::<i32>().unwrap();
                let z = split.next().unwrap().parse::<i32>().unwrap();
                scanners.last_mut().unwrap().push((x, y, z))
            }
        }
    }
}

fn main() {
    let mut scanners: Vec<Vec<(i32, i32, i32)>> = Vec::new();
    // let's start with a simple vec. Optimize later if necessary
    let mut beacon_list: Vec<(i32, i32, i32)> = Vec::new();

    let lines: Vec<&str> = include_str!("../../data/day19/test.txt").lines().collect();
    for line in lines {
        extract_scanners(line, &mut scanners);
    }

    println!("Found {} scanners.", scanners.len());

    // // extract scanner 0's beacons as the initial set
    // for beacon in &scanners[0] {
    //     beacon_list.push(*beacon);
    // }
    // scanners.swap_remove(0);

    // for rotation in 0..24 {
    //     println!("Rotation {}", rotation);
    //     let mut scanners_to_remove: Vec<usize> = Vec::new();
    //     let mut current_scanner: usize = 0;
    //     for scanner in &scanners {
    //         let mut num_matches = 0;
    //         let mut rotated_non_matching_beacons = Vec::new();
    //         for beacon in scanner {
    //             let result = rotate_and_match_beacon(&beacon, &rotation, &beacon_list);
    //             num_matches = result.0;
    //             rotated_non_matching_beacons = result.1;
    //         }
    //         if num_matches >= 12 {
    //             for beacon in &rotated_non_matching_beacons {
    //                 beacon_list.push(*beacon);
    //             }
    //             scanners_to_remove.push(current_scanner);
    //         }
    //         current_scanner += 1;
    //     }
    //     for scanner_idx in &scanners_to_remove {
    //         scanners.swap_remove(*scanner_idx);
    //     }
    // }

    // println!("Found {} beacons.", beacon_list.len());
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
struct Position {
    x: i32,
    y: i32,
    z: i32,
}

impl Position {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Position { x, y, z }
    }

    pub fn distance(&self, other: &Position) -> f32 {
        let squares: f32 = (self.x - other.x) as f32 * (self.x - other.x) as f32
            + (self.y - other.y) as f32 * (self.y - other.y) as f32;
        squares.sqrt()
    }

    pub fn minus(&self, other: &Position) -> Position {
        Position {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

struct Scanner {
    beacons: Vec<Position>,
}

impl Scanner {
    pub fn new(beacons: impl Iterator<Item = Position>) -> Self {
        Scanner {
            beacons: beacons.collect(),
        }
    }
    pub fn len(&self) -> usize {
        self.beacons.len()
    }
    pub fn iter(&self) -> impl Iterator<Item = &Position> {
        self.beacons.iter()
    }
}

fn rotate(rotation: u8, pos: &Position) -> Position {
    let (new_x, new_y, new_z) = match rotation {
        0 => (pos.x, pos.y, pos.z),
        1 => (pos.x, -pos.z, pos.y),
        2 => (pos.x, -pos.y, -pos.z),
        3 => (pos.x, pos.z, -pos.y),
        4 => (-pos.x, -pos.y, pos.z),
        5 => (-pos.x, -pos.z, -pos.y),
        6 => (-pos.x, pos.y, -pos.z),
        7 => (-pos.x, pos.z, pos.y),
        8 => (-pos.y, pos.x, pos.z),
        9 => (pos.z, pos.x, pos.y),
        10 => (pos.y, pos.x, -pos.z),
        11 => (-pos.z, pos.x, -pos.y),
        12 => (-pos.y, -pos.x, pos.z),
        13 => (pos.z, -pos.x, pos.y),
        14 => (pos.y, -pos.x, -pos.z),
        15 => (-pos.z, -pos.x, -pos.y),
        16 => (-pos.y, pos.z, pos.x),
        17 => (pos.z, pos.y, pos.x),
        18 => (pos.y, -pos.z, pos.x),
        19 => (-pos.z, -pos.y, pos.x),
        20 => (-pos.y, pos.z, -pos.x),
        21 => (pos.z, pos.y, -pos.x),
        22 => (pos.y, -pos.z, -pos.x),
        23 => (-pos.z, -pos.y, -pos.x),
        _ => panic!("Unexpected rotation"),
    };
    Position {
        x: new_x,
        y: new_y,
        z: new_z,
    }
}

fn overlap(scanner_a: Scanner, scanner_b: Scanner, min_overlap: usize) -> Option<Position> {
    for rotation in 0..24 {
        let mut distances = vec![vec![Position::new(0, 0, 0); scanner_b.len()]; scanner_a.len()];
        for (idx_a, a) in scanner_a.iter().enumerate() {
            for (idx_b, b) in scanner_b.iter().enumerate() {
                distances[idx_a][idx_b] = a.minus(&rotate(rotation, b));
            }
        }

        for (row_idx, check_row) in distances.iter().enumerate() {
            for (idx, check_distance) in check_row.iter().enumerate() {
                let mut num_found = 1;
                for row in &distances[(row_idx + 1)..] {
                    for col in (idx + 1)..row.len() {
                        if row[col] == *check_distance {
                            num_found += 1;
                            break;
                        }
                    }
                    if num_found >= min_overlap {
                        println!("Found some for rotation {}", rotation);
                        return Some(*check_distance);
                    }
                }
                if num_found >= min_overlap {
                    println!("Found some for rotation {}", rotation);
                    return Some(*check_distance);
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::{overlap, Position, Scanner};

    #[test]
    fn can_create_scanner() {
        let _ = Scanner::new(std::iter::empty());
    }

    #[test]
    fn empty_scanners_dont_overlap() {
        let scanner_a = Scanner::new(std::iter::empty());
        let scanner_b = Scanner::new(std::iter::empty());
        let relative_pos = overlap(scanner_a, scanner_b, 1);
        assert!(relative_pos.is_none());
    }

    #[test]
    fn single_beacon_scanners_overlap_trivially() {
        let scanner_a = Scanner::new(std::iter::once(Position::new(0, 0, 0)));
        let scanner_b = Scanner::new(std::iter::once(Position::new(0, 0, 0)));
        let distance = overlap(scanner_a, scanner_b, 1);
        assert!(distance.is_some());
        assert_eq!(distance.unwrap().x, 0);
    }

    #[test]
    fn single_beacon_scanners_overlap_non_trivially() {
        let scanner_a = Scanner::new(std::iter::once(Position::new(1, 0, 0)));
        let scanner_b = Scanner::new(std::iter::once(Position::new(0, 0, 0)));
        let distance = overlap(scanner_a, scanner_b, 1);
        assert!(distance.is_some());
        assert_eq!(distance.unwrap().x, 1);
    }

    #[test]
    fn two_beacon_scanners_dont_overlap() {
        let positions_a = vec![Position::new(0, 0, 0), Position::new(1, 0, 0)];
        let positions_b = vec![Position::new(0, 0, 0), Position::new(2, 0, 0)];
        let scanner_a = Scanner::new(positions_a.into_iter());
        let scanner_b = Scanner::new(positions_b.into_iter());
        let distance = overlap(scanner_a, scanner_b, 2);
        assert!(distance.is_none());
    }

    #[test]
    fn two_beacon_scanners_overlap() {
        let positions_a = vec![Position::new(0, 0, 0), Position::new(1, 0, 0)];
        let positions_b = vec![Position::new(1, 0, 0), Position::new(2, 0, 0)];
        let scanner_a = Scanner::new(positions_a.into_iter());
        let scanner_b = Scanner::new(positions_b.into_iter());
        let distance = overlap(scanner_a, scanner_b, 2);
        assert!(distance.is_some());
        assert_eq!(distance.unwrap().x, -1);
    }

    #[test]
    fn three_beacon_scanners_overlap_in_two() {
        let positions_a = vec![
            Position::new(0, 0, 0),
            Position::new(1, 0, 0),
            Position::new(4, 0, 0),
        ];
        let positions_b = vec![
            Position::new(10, 0, 0),
            Position::new(11, 0, 0),
            Position::new(13, 0, 0),
        ];
        let scanner_a = Scanner::new(positions_a.into_iter());
        let scanner_b = Scanner::new(positions_b.into_iter());
        let distance = overlap(scanner_a, scanner_b, 2);
        assert!(distance.is_some());
        assert_eq!(distance.unwrap().x, -10);
    }

    #[test]
    fn three_beacon_scanners_overlap_in_two_with_offset() {
        let positions_a = vec![
            Position::new(0, 0, 0),
            Position::new(1, 0, 0),
            Position::new(3, 0, 0),
        ];
        let positions_b = vec![
            Position::new(7, 0, 0),
            Position::new(11, 0, 0),
            Position::new(13, 0, 0),
        ];
        let scanner_a = Scanner::new(positions_a.into_iter());
        let scanner_b = Scanner::new(positions_b.into_iter());
        let distance = overlap(scanner_a, scanner_b, 2);
        assert!(distance.is_some());
        assert_eq!(distance.unwrap().x, -10);
    }

    #[test]
    fn three_beacon_scanners_overlap_in_two_with_more_offset() {
        let positions_a = vec![
            Position::new(0, 0, 0),
            Position::new(2, 0, 0),
            Position::new(5, 0, 0),
        ];
        let positions_b = vec![
            Position::new(8, 0, 0),
            Position::new(9, 0, 0),
            Position::new(22, 0, 0),
            Position::new(25, 0, 0),
        ];
        let scanner_a = Scanner::new(positions_a.into_iter());
        let scanner_b = Scanner::new(positions_b.into_iter());
        let distance = overlap(scanner_a, scanner_b, 2);
        assert!(distance.is_some());
        assert_eq!(distance.unwrap().x, -20);
    }

    #[test]
    fn first_test_with_2_d_coordinates() {
        let positions_a = vec![
            Position::new(0, 2, 0),
            Position::new(4, 1, 0),
            Position::new(3, 3, 0),
        ];
        let positions_b = vec![
            Position::new(-1, -1, 0),
            Position::new(-5, 0, 0),
            Position::new(-2, 1, 0),
        ];
        let scanner_a = Scanner::new(positions_a.into_iter());
        let scanner_b = Scanner::new(positions_b.into_iter());
        let distance = overlap(scanner_a, scanner_b, 2);
        assert!(distance.is_some());
        assert_eq!(distance.unwrap(), Position { x: 5, y: 2, z: 0 });
    }

    #[test]
    fn first_test_with_rotated_2_d_coordinates() {
        let positions_a = vec![
            Position::new(0, 0, 0),
            Position::new(1, 0, 0),
            Position::new(3, 0, 0),
        ];
        let positions_b = vec![
            Position::new(0, -2, 0),
            Position::new(0, -3, 0),
            Position::new(0, -5, 0),
        ];
        let scanner_a = Scanner::new(positions_a.into_iter());
        let scanner_b = Scanner::new(positions_b.into_iter());
        let distance = overlap(scanner_a, scanner_b, 3);
        assert!(distance.is_some());
        assert_eq!(distance.unwrap(), Position { x: -2, y: 0, z: 0 });
    }

    #[test]
    fn rotation_test_from_aoc_description() {
        let positions_a = vec![
            Position::new(-1, -1, 1),
            Position::new(-2, -2, 2),
            Position::new(-3, -3, 3),
            Position::new(-2, -3, 1),
            Position::new(5, 6, -4),
            Position::new(8, 0, 7),
        ];
        let positions_b = vec![
            Position::new(1, -1, 1),
            Position::new(2, -2, 2),
            Position::new(3, -3, 3),
            Position::new(2, -1, 3),
            Position::new(-5, 4, -6),
            Position::new(-8, -7, 0),
        ];
        let scanner_a = Scanner::new(positions_a.into_iter());
        let scanner_b = Scanner::new(positions_b.into_iter());
        let distance = overlap(scanner_a, scanner_b, 6);
        assert!(distance.is_some());
        assert_eq!(distance.unwrap(), Position { x: 0, y: 0, z: 0 });
    }

    #[test]
    fn first_test_with_3_d_coordinates() {
        let positions_a = vec![
            Position::new(404, -588, -901),
            Position::new(528, -643, 409),
            Position::new(-838, 591, 734),
            Position::new(390, -675, -793),
            Position::new(-537, -823, -458),
            Position::new(-485, -357, 347),
            Position::new(-345, -311, 381),
            Position::new(-661, -816, -575),
            Position::new(-876, 649, 763),
            Position::new(-618, -824, -621),
            Position::new(553, 345, -567),
            Position::new(474, 580, 667),
            Position::new(-447, -329, 318),
            Position::new(-584, 868, -557),
            Position::new(544, -627, -890),
            Position::new(564, 392, -477),
            Position::new(455, 729, 728),
            Position::new(-892, 524, 684),
            Position::new(-689, 845, -530),
            Position::new(423, -701, 434),
            Position::new(7, -33, -71),
            Position::new(630, 319, -379),
            Position::new(443, 580, 662),
            Position::new(-789, 900, -551),
            Position::new(459, -707, 401),
        ];
        let positions_b = vec![
            Position::new(686, 422, 578),
            Position::new(605, 423, 415),
            Position::new(515, 917, -361),
            Position::new(-336, 658, 858),
            Position::new(95, 138, 22),
            Position::new(-476, 619, 847),
            Position::new(-340, -569, -846),
            Position::new(567, -361, 727),
            Position::new(-460, 603, -452),
            Position::new(669, -402, 600),
            Position::new(729, 430, 532),
            Position::new(-500, -761, 534),
            Position::new(-322, 571, 750),
            Position::new(-466, -666, -811),
            Position::new(-429, -592, 574),
            Position::new(-355, 545, -477),
            Position::new(703, -491, -529),
            Position::new(-328, -685, 520),
            Position::new(413, 935, -424),
            Position::new(-391, 539, -444),
            Position::new(586, -435, 557),
            Position::new(-364, -763, -893),
            Position::new(807, -499, -711),
            Position::new(755, -354, -619),
            Position::new(553, 889, -390),
        ];
        let scanner_a = Scanner::new(positions_a.into_iter());
        let scanner_b = Scanner::new(positions_b.into_iter());
        let distance = overlap(scanner_a, scanner_b, 12);
        assert!(distance.is_some());
        assert_eq!(
            distance.unwrap(),
            Position {
                x: 68,
                y: -1246,
                z: -435
            }
        );
    }
}

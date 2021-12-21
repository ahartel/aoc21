
fn to_bin_string(c: char) -> String {
    match c {
        '0' => "0000".to_string(),
        '1' => "0001".to_string(),
        '2' => "0010".to_string(),
        '3' => "0011".to_string(),
        '4' => "0100".to_string(),
        '5' => "0101".to_string(),
        '6' => "0110".to_string(),
        '7' => "0111".to_string(),
        '8' => "1000".to_string(),
        '9' => "1001".to_string(),
        'A' => "1010".to_string(),
        'B' => "1011".to_string(),
        'C' => "1100".to_string(),
        'D' => "1101".to_string(),
        'E' => "1110".to_string(),
        'F' => "1111".to_string(),
        _ => panic!("unexpected character"),
    }
}

#[derive(PartialEq)]
enum ParseState {
    Version,
    Type,
    Literal,
    LengthTypeID,
    LengthOrNum,
    End
}

#[derive(Clone)]
struct Packet {
    version: usize,
    is_operator: bool,
    is_length_not_num: bool,
    length_or_num: usize,
    children_start_pos: usize,
    depth: usize,
    num_children_found: usize
}

fn get_version_sum(s: &String) -> usize {
    let mut state = ParseState::Version;
    let mut idx = 0;
    let mut packets : Vec<Packet> = Vec::new();
    let mut depth = 0;
    let mut cur_packet = Packet {
        version: 0,
        is_operator: false,
        is_length_not_num: false,
        length_or_num: 0,
        children_start_pos: 0,
        depth: 0,
        num_children_found: 0
    };

    while idx < s.len() {
        match state {
            ParseState::Version => {
                cur_packet.version = usize::from_str_radix(&s[idx..idx+3], 2).unwrap();
                state = ParseState::Type;
                idx += 3;
            },
            ParseState::Type => {
                let raw_type = usize::from_str_radix(&s[idx..idx+3], 2).unwrap();
                cur_packet.is_operator = raw_type != 4;
                idx += 3;
                if cur_packet.is_operator {
                    state = ParseState::LengthTypeID;
                }
                else {
                    state = ParseState::Literal;
                }
            }
            ParseState::Literal => {
                if &s[idx..idx+1] == "1" {
                    state = ParseState::Literal;
                }
                else {
                    cur_packet.depth = depth;
                    state = ParseState::End;
                }
                // ignore content for now
                idx += 5;
            },
            ParseState::LengthTypeID => {
                cur_packet.is_length_not_num = &s[idx..idx+1] == "0";
                idx += 1;
                state = ParseState::LengthOrNum;
            },
            ParseState::LengthOrNum => {
                let field_size = if cur_packet.is_length_not_num { 15 } else { 11 };
                cur_packet.length_or_num = usize::from_str_radix(&s[idx..idx+field_size], 2).unwrap();
                idx += field_size;
                cur_packet.children_start_pos = idx;
                cur_packet.depth = depth;
                depth += 1;
                state = ParseState::End;
            },
            ParseState::End => {
                let parent_packet = packets.iter_mut().rev().filter(|p| p.is_operator && depth > 0 && p.depth == depth-1).nth(0);   
                match parent_packet {
                    None => {
                        packets.push(cur_packet.clone());
                    },
                    Some(packet) => {
                        if packet.is_operator && packet.is_length_not_num {
                            packet.num_children_found += 1;
                            let final_pos_reached = idx - packet.children_start_pos == packet.length_or_num;
                            packets.push(cur_packet.clone());
                            if final_pos_reached && depth > 0 {
                                depth -= 1;
                            }
                        }
                        else if packet.is_operator && !packet.is_length_not_num {
                            packet.num_children_found += 1;
                            let num_packets_reached = packet.num_children_found == packet.length_or_num;
                            packets.push(cur_packet.clone());
                            if num_packets_reached && depth > 0 {
                                depth -= 1;
                            }
                        }
                        else { // not an operator
                            packets.push(cur_packet.clone());
                        }
                    }
                }
                state = ParseState::Version;
                if s.len() - idx < 8 && state == ParseState::Version {
                    idx = s.len()
                }
            },
        }
    }
    let version_sum = packets.iter().map(|p| p.version).sum();
    println!("Version sum of packet: {}", version_sum);
    version_sum
}

fn main() {
    let binary_lines : Vec<usize> = include_str!("../../data/day16/input.txt")
        .lines()
        .map(|l| l.chars().map(|c| to_bin_string(c)).collect::<Vec<String>>().join(""))
        .map(|s| get_version_sum(&s))
        .collect();
    println!("{:?}", binary_lines);
    println!("{}", binary_lines.iter().sum::<usize>());
}

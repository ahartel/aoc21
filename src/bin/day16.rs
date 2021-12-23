use std::cmp;

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
    typ: usize,
    is_operator: bool,
    is_length_not_num: bool,
    length_or_num: usize,
    literal_value: String,
    children_start_pos: usize,
    depth: usize,
    start_idx: usize,
    children: Vec<Packet>
}

impl Packet {
    fn new() -> Packet {
        Packet {
            version: 0,
            typ: 0,
            is_operator: false,
            is_length_not_num: false,
            length_or_num: 0,
            literal_value: String::new(),
            children_start_pos: 0,
            depth: 0,
            start_idx: 0,
            children: Vec::new()
        }
    }
}

fn interpret_packets(s: &String, start_idx: usize, depth: usize, parent: &mut Packet) -> usize {
    let mut state = ParseState::Version;
    let mut idx = start_idx;
    let mut cur_packet = Packet::new();

    loop {
        match state {
            ParseState::Version => {
                cur_packet.version = usize::from_str_radix(&s[idx..idx+3], 2).unwrap();
                state = ParseState::Type;
                cur_packet.start_idx = idx;
                idx += 3;
            },
            ParseState::Type => {
                let raw_type = usize::from_str_radix(&s[idx..idx+3], 2).unwrap();
                cur_packet.is_operator = raw_type != 4;
                cur_packet.typ = raw_type;
                idx += 3;
                if cur_packet.is_operator {
                    state = ParseState::LengthTypeID;
                }
                else {
                    cur_packet.literal_value.clear();
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
                cur_packet.literal_value.push_str(&s[idx+1..idx+5]);
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
                state = ParseState::End;
            },
            ParseState::End => {

                if cur_packet.is_operator && cur_packet.is_length_not_num {
                    //packet.num_children_found += 1;
                    while idx - cur_packet.children_start_pos < cur_packet.length_or_num {
                        idx = interpret_packets(s, idx, depth+1, &mut cur_packet);
                    }
                }
                else if cur_packet.is_operator && !cur_packet.is_length_not_num {
                    //packet.num_children_found += 1;
                    while cur_packet.children.len() < cur_packet.length_or_num {
                        idx = interpret_packets(s, idx, depth+1, &mut cur_packet)
                    }
                }
                else { // not an operator
                }

                parent.children.push(cur_packet.clone());

                break;
            },
        }
    }
    idx
}

fn execute_packets(packet: &Packet) -> usize {
    match packet.typ {
        0 => { // sum
            let mut sum = 0;
            for child in &packet.children {
                sum += execute_packets(&child);
            }
            sum
        },
        1 => { // product
            let mut product = 1;
            for child in &packet.children {
                product *= execute_packets(&child);
            }
            product
        },
        2 => { // min
            let mut values = Vec::new();
            for child in &packet.children {
                values.push(execute_packets(&child));
            }
            *values.iter().min().unwrap()
        },
        3 => { // max
            let mut values = Vec::new();
            for child in &packet.children {
                values.push(execute_packets(&child));
            }
            *values.iter().max().unwrap()
        },
        4 => { // literal
            usize::from_str_radix(&packet.literal_value, 2).unwrap()
        },
        5 => { // greater than
            let mut values = Vec::new();
            for child in &packet.children {
                values.push(execute_packets(&child));
            }
            (values[0] > values[1]) as usize
        },
        6 => { // less than
            let mut values = Vec::new();
            for child in &packet.children {
                values.push(execute_packets(&child));
            }
            (values[0] < values[1]) as usize
        },
        7 => { // equal to
            let mut values = Vec::new();
            for child in &packet.children {
                values.push(execute_packets(&child));
            }
            (values[0] == values[1]) as usize
        },
        _ => panic!("Unknown packet type")
    }
}

fn print_tree(node: &Packet) {
    if node.is_operator {
        if node.is_length_not_num {
            println!(" {: <0$}Type: {}, Start Index: {}, Childrenlen: {}, #Children: {}", node.depth, node.typ, node.start_idx, node.length_or_num, node.children.len());
        }
        else {
            println!(" {: <0$}Type: {}, Start Index: {}, Numchildren: {}, #Children: {}", node.depth, node.typ, node.start_idx, node.length_or_num, node.children.len());
        }
    }
    else {
        println!(" {: <0$}Type: {}, Value: {}, Start Index: {}", node.depth, node.typ, usize::from_str_radix(&node.literal_value, 2).unwrap(), node.start_idx);
    }
    for child in &node.children {
        print_tree(&child);
    }
}

fn get_version_sum(node: &Packet) -> usize {
    let mut version_sum = node.version;
    for child in &node.children {
        version_sum += get_version_sum(&child);
    }
    version_sum
}

fn main() {
    let binary_lines : Vec<String> = include_str!("../../data/day16/input.txt")
        .lines()
        .map(|l| l.chars().map(|c| to_bin_string(c)).collect::<Vec<String>>().join(""))
        .collect();
    for line in binary_lines {
        println!("Line: {}...", &line[0..cmp::min(50, line.len())]);

        let shorter_line = line;
        let mut root_node = Packet::new();
        interpret_packets(&shorter_line, 0, 0, &mut root_node);

        println!("Packets:");
        print_tree(&root_node.children[0]);

        let version_sum : usize = get_version_sum(&root_node.children[0]);
        println!(" Version sum of packet: {}", version_sum);
        
        let result = execute_packets(&root_node.children[0]);
        println!(" Result: {}", result);
    }
}

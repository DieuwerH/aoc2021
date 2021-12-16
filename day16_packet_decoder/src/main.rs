use std::fs;

fn main() {
    let input = read_input(false);
    // let input = read_input(false);
    part_one(&input);
}

fn read_input(use_example: bool) -> String {
    let mut filename = "./src/input.txt";

    if use_example {
        filename = "./src/example.txt";
    }

    let res = fs::read_to_string(filename).unwrap();

    res
}

fn part_one(input: &String) {
    let string = input;
    let mut res = Vec::new();
    for chr in string.chars() {
        let mut bits = hex_to_bits(chr);
        res.append(&mut bits);
    }
    println!("{}", parse_packet(&mut res));
}

type Packet = Vec<u8>;
fn take_first(num: usize, p: &mut Packet) -> Vec<u8> {
    p.drain(..num).collect()
}

fn parse_packet(packet: &mut Packet) -> usize {
    let (_, type_id) = parse_header_and_type(packet);
    let res = match type_id {
        4 => parse_literal(packet),
        _ => parse_operator(packet, type_id),
    };
    res
}

fn hex_to_bits(hex: char) -> Vec<u8> {
    let res = match hex {
        '0' => vec![0, 0, 0, 0],
        '1' => vec![0, 0, 0, 1],
        '2' => vec![0, 0, 1, 0],
        '3' => vec![0, 0, 1, 1],
        '4' => vec![0, 1, 0, 0],
        '5' => vec![0, 1, 0, 1],
        '6' => vec![0, 1, 1, 0],
        '7' => vec![0, 1, 1, 1],
        '8' => vec![1, 0, 0, 0],
        '9' => vec![1, 0, 0, 1],
        'A' => vec![1, 0, 1, 0],
        'B' => vec![1, 0, 1, 1],
        'C' => vec![1, 1, 0, 0],
        'D' => vec![1, 1, 0, 1],
        'E' => vec![1, 1, 1, 0],
        'F' => vec![1, 1, 1, 1],
        _ => Vec::new(),
    };
    res
}

fn parse_header_and_type(packet: &mut Packet) -> (usize, usize) {
    let header = binary_parser(&take_first(3, packet));
    let type_id = binary_parser(&take_first(3, packet));
    (header, type_id)
}

fn binary_parser(bin_vec: &[u8]) -> usize {
    let mut res = 0usize;
    let len = bin_vec.len() - 1;
    for (i, bin) in bin_vec.iter().enumerate() {
        if bin == &1 {
            let cur = usize::pow(2, (len - i) as u32);
            res += cur;
        }
    }
    res
}

fn parse_literal(literal: &mut Packet) -> usize {
    let mut res = Vec::new();

    let mut part = take_first(5, literal);
    let mut should_stop = false;
    while !should_stop {
        res.append(&mut part[1..].to_vec());
        if part[0] == 0 {
            should_stop = true;
        } else {
            part = take_first(5, literal);
        }
    }
    binary_parser(&res)
}

fn parse_operator(operator: &mut Packet, type_id: usize) -> usize {
    let lenght_type_id = take_first(1, operator);

    let mut nums = Vec::new();
    if lenght_type_id[0] == 0 {
        let total_bit_length = binary_parser(&take_first(15, operator));
        let start_len = operator.len();
        while start_len - operator.len() < total_bit_length {
            nums.push(parse_packet(operator));
        }
    } else {
        let num_sub_packets = binary_parser(&take_first(11, operator));
        for _ in 0..num_sub_packets {
            nums.push(parse_packet(operator));
        }
    }
    match type_id {
        0 => nums.iter().sum(),
        1 => nums.iter().product(),
        2 => *nums.iter().min().unwrap(),
        3 => *nums.iter().max().unwrap(),
        5 => gt(nums),
        6 => lt(nums),
        7 => eq(nums),
        _ => 0,
    }
}

fn gt(p: Vec<usize>) -> usize {
    if p[0] > p[1] {
        return 1;
    } else {
        0
    }
}

fn lt(p: Vec<usize>) -> usize {
    if p[0] < p[1] {
        return 1;
    }
    0
}

fn eq(p: Vec<usize>) -> usize {
    if p[0] == p[1] {
        return 1;
    }
    0
}

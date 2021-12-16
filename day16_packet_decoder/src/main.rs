use std::fs;

fn main() {
    let input = read_input(true);
    // let input = read_input(false);
    part_one(&input);
    part_two(&input);
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
    let string = "D2FE28";
    let mut res = Vec::new();
    for chr in string.chars() {
        let mut bits = hex_to_bits(chr);
        res.append(&mut bits);
    }
    println!("{:?}", res)
}
fn part_two(input: &String) {}

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
        'f' => vec![1, 1, 1, 1],
        _ => Vec::new(),
    };
    res
}

fn parse_header_and_type(packet: &[u8]) -> &[u8] {
    let header = &packet[0..2];
    let type_id = &packet[2..5];
    let rest = &packet[5..];

    rest
}

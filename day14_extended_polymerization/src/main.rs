use std::fs;

fn main() {
    let input = read_input(true);
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
    println!("Input:\n{}", input)
}

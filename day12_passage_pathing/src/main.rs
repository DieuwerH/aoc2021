use std::fs;

fn main() {
    let input = read_input(Some(true));
    part_one(&input);
    part_two(&input);
}

fn read_input(example: Option<bool>) -> String {
    let read_example = example.unwrap_or(false);
    let mut filename = "src/input.txt";
    if read_example {
        filename = "src/example.txt"
    }
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    contents
}

struct Node {
    name: String,
    edges: Vec<Node>,
}

fn part_one(input: &String) {
    println!("PART ONE");
    println!("{}", input);

    let graph: Vec<Node> = Vec::new();
    for line in input.lines() {
        let names = line.split("-");
        for name in names {
            let ns = String::from(name);
            for node in &graph {
                if node.name 
            }
        }
    }
}
fn part_two(input: &String) {
    println!("PART ONE");
    println!("{}", input)
}

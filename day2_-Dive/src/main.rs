use std::fs;

fn main() {
    println!("PART 1");
    part_one();
    println!("PART 2");
    part_two();
}

fn part_one() {
    let contents =
        fs::read_to_string("src/input.txt").expect("Something went wrong reading the file");
    let instructions = contents.lines();

    let mut h = 0;
    let mut d = 0;

    for ins in instructions {
        let ins: Vec<&str> = ins.split(" ").collect();
        let dir = ins[0];
        let amount = ins[1].parse::<u32>().unwrap();
        match dir {
            "forward" => h += amount,
            "down" => d += amount,
            "up" => d -= amount,
            _ => d = d,
        }
    }
    println!("Horizontal:\t {}", &h);
    println!("Depth:\t\t {}", &d);
    println!("Multiplied:\t {}", &h * &d);
}

fn part_two() {
    let contents =
        fs::read_to_string("src/input.txt").expect("Something went wrong reading the file");
    let instructions = contents.lines();

    let mut h = 0;
    let mut d = 0;
    let mut a = 0;

    for ins in instructions {
        let ins: Vec<&str> = ins.split(" ").collect();
        let dir = ins[0];
        let amount = ins[1].parse::<u32>().unwrap();
        match dir {
            "forward" => {
                h += amount;
                d += a * amount;
            }
            "down" => a += amount,
            "up" => a -= amount,
            _ => d = d,
        }
    }
    println!("Horizontal:\t {}", &h);
    println!("Depth:\t\t {}", &d);
    println!("Multiplied:\t {}", &h * &d);
}

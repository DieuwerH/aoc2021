use std::fs;

fn main() {
    let input = read_input(Some(false));
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

fn part_one(input: &String) {
    println!("=== PART ONE: ");
    let mut fishes = input
        .split(",")
        .map(|str_num| str_num.parse::<i8>().unwrap())
        .collect::<Vec<i8>>();
    print!("Initial state:  ");
    print!("{} fishes:\t", fishes.len());
    print_fishes(&fishes);

    for i in 0..18 {
        simulate(&mut fishes);
        // print!("After {} days:\t", i + 1);
        // print!("{} fishes:\t", fishes.len());
        // print_fishes(&fishes);
    }
    print!("After 80 days:\t");
    print!("{} fishes:\t\n", fishes.len());
}
fn part_two(input: &String) {
    println!("=== PART TWO: ");
    let mut fishes = input
        .split(",")
        .map(|str_num| str_num.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    print!("Initial state:  ");
    print!("{} fishes:\t", fishes.len());
    // print_fishes(&fishes);
    let mut counter = [0u64; 9];
    for fish in fishes {
        counter[fish] += 1;
    }

    for i in 0..256 {
        let tmp = counter[0];
        for i in 0..counter.len() - 1 {
            counter[i] = counter[i + 1];
        }
        counter[counter.len() - 1] = tmp;
        counter[6] += tmp;
        println!(
            "After day {} #fishes: {}\t{:?}",
            (i + 1),
            counter.iter().sum::<u64>(),
            counter
        );
    }
}

fn print_fishes(fishes: &Vec<i8>) {
    for fish in fishes {
        print!("{} ", fish)
    }
    print!("\n");
}

fn simulate(fishes: &mut Vec<i8>) {
    for i in 0..fishes.len() {
        if fishes[i] == 0 {
            fishes[i] = 7;
            fishes.push(8);
        }
        fishes[i] = fishes[i] - 1;
    }
}

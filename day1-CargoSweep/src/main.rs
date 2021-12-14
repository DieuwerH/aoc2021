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
    let mut measurements = contents.lines();
    let mut current = measurements.next().unwrap().parse::<u32>().unwrap();
    let mut counter = 0;
    for meas in measurements {
        println!("meas {}", meas);
        let other = meas.parse::<u32>().unwrap();
        if other > current {
            counter += 1;
        }
        current = other;
    }
    println!("Count: {}", counter);
}

fn part_two() {
    let contents =
        fs::read_to_string("src/input.txt").expect("Something went wrong reading the file");
    let measurements: Vec<u32> = contents
        .lines()
        .map(|l| l.parse::<u32>().unwrap())
        .collect();
    const WINSIZE: usize = 3;
    let mut current = get_window(&measurements, 0, WINSIZE);
    let mut counter = 0;
    for i in 1..measurements.len() - 2 {
        // println!("Meas: {}", &measurements[i]);
        let other = get_window(&measurements, i, WINSIZE);
        if other > current {
            counter += 1;
        }
        current = other;
    }
    println!("Count: {}", counter);
}

fn get_window(measurements: &Vec<u32>, index: usize, win_size: usize) -> u32 {
    let mut res = 0;
    for i in 0..win_size {
        res += measurements[index + i];
    }
    res
}

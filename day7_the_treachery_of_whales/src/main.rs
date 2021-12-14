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
    let numbers = input
        .split(",")
        .map(|num_string| num_string.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    println!("Crab locations: {:?}", numbers);
    let total: i32 = numbers.iter().sum();
    let avg = total / numbers.len() as i32;
    println!("Average location: {}", avg);

    let mut lh: i32 = avg;
    let mut min_cost = fuel_cost(lh, &numbers);
    while true {
        let cost_a = fuel_cost(lh - 1, &numbers);
        let cost_b = fuel_cost(lh + 1, &numbers);
        if cost_a < min_cost && cost_a < cost_b {
            lh = lh - 1;
            min_cost = cost_a;
        } else if cost_b < min_cost && cost_b < cost_a {
            lh = lh + 1;
            min_cost = cost_b;
        } else {
            break;
        }
    }
    println!("Line at {} produces minimal cost {}", lh, min_cost);
}
fn part_two(input: &String) {
    let numbers = input
        .split(",")
        .map(|num_string| num_string.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    println!("Crab locations: {:?}", numbers);
    let total: i32 = numbers.iter().sum();
    let avg = total / numbers.len() as i32;
    println!("Average location: {}", avg);

    let mut lh: i32 = avg;
    let mut min_cost = expansive_fuel_cost(lh, &numbers);
    while true {
        let cost_a = expansive_fuel_cost(lh - 1, &numbers);
        let cost_b = expansive_fuel_cost(lh + 1, &numbers);
        if cost_a < min_cost && cost_a < cost_b {
            lh = lh - 1;
            min_cost = cost_a;
        } else if cost_b < min_cost && cost_b < cost_a {
            lh = lh + 1;
            min_cost = cost_b;
        } else {
            break;
        }
    }
    println!("Line at {} produces minimal cost {}", lh, min_cost);
}

fn fuel_cost(line_height: i32, crabs: &Vec<i32>) -> i32 {
    let mut cost = 0i32;
    for crab in crabs {
        cost += i32::abs(line_height - *crab)
    }
    // println!("Cost for line at {}: {}", line_height, cost);
    cost
}

fn expansive_fuel_cost(line_height: i32, crabs: &Vec<i32>) -> i32 {
    let mut cost = 0i32;
    for crab in crabs {
        let dist = i32::abs(line_height - crab);
        let cur_cost = dist * (dist + 1) / 2;
        cost += cur_cost;
    }
    // println!("Cost for line at {}: {}", line_height, cost);
    cost
}

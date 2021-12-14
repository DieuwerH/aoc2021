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
    // let input = "[({(<(())[]>[[{[]{<()<>>";
    println!("PART ONE");
    let mut score = 0;
    for line in input.lines() {
        // println!("Parsing {}", line);
        let parts: Vec<&str> = line.split_terminator("").skip(1).collect();
        score += parse(&parts);
    }
    println!("Score {}", score);
}

fn part_two(input: &String) {
    println!("PART TWO");

    let mut scores = Vec::new();

    for line in input.lines() {
        // println!("Parsing {}", line);
        let parts: Vec<&str> = line.split_terminator("").skip(1).collect();
        let mut tape = Vec::new();

        let mut skip = false;

        for part in parts {
            if "([{<".contains(part) {
                tape.push(part);
            } else if ")]}>".contains(part) {
                if tape.len() == 0 {
                    continue;
                } else {
                    let opener = tape.pop().unwrap();

                    if part != closer(opener) {
                        // println!("\tExpected {}, but found {} instead", closer(opener), part);
                        skip = true;
                        break;
                    }
                }
            }
        }

        if skip {
            continue;
        }
        let mut score = 0u64;
        while tape.len() > 0 {
            score *= 5;
            let opener = tape.pop().unwrap();
            score += score2(closer(opener));
        }
        // println!("Score {}", score);
        scores.push(score);
    }
    scores.sort();
    println!("Middle score: {}", scores[scores.len() / 2])
}

fn parse(input: &Vec<&str>) -> u32 {
    let mut tape = Vec::new();

    for part in input {
        if "([{<".contains(part) {
            tape.push(part);
        } else if ")]}>".contains(part) {
            if tape.len() == 0 {
                return 0;
            } else {
                let opener = tape.pop().unwrap();

                if part != &closer(opener) {
                    // println!("\tExpected {}, but found {} instead", closer(opener), part);
                    return score(part);
                }
            }
        }
        // println!("Tape: {:?}", tape);
    }

    0
}

fn closer(opener: &str) -> &str {
    if opener == "(" {
        return ")";
    } else if opener == "{" {
        return "}";
    } else if opener == "[" {
        return "]";
    } else if opener == "<" {
        return ">";
    }
    "-"
}

fn score(ill_char: &str) -> u32 {
    match ill_char {
        ")" => 3,
        "]" => 57,
        "}" => 1197,
        ">" => 25137,
        _ => 0,
    }
}

fn score2(ill_char: &str) -> u64 {
    match ill_char {
        ")" => 1,
        "]" => 2,
        "}" => 3,
        ">" => 4,
        _ => 0,
    }
}

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
    const THRESHOLD: u8 = 2u8;
    println!(
        "==== PART ONE: FINDING DANGEROUS AREAS WITH DANGER THRESHOLD {}",
        THRESHOLD
    );
    let (max_x, max_y, lin_segs) = find_max_values(&input);
    let mut diagram = vec![vec![0u8; (max_x + 1) as usize]; (max_y + 1) as usize];
    let mut danger_counter = 0;
    for lin_seg in lin_segs {
        if lin_seg.a.x == lin_seg.b.x {
            let x = lin_seg.a.x;
            for y in
                std::cmp::min(lin_seg.a.y, lin_seg.b.y)..=std::cmp::max(lin_seg.a.y, lin_seg.b.y)
            {
                diagram[y as usize][x as usize] += 1;
                if diagram[y as usize][x as usize] == THRESHOLD {
                    danger_counter += 1;
                }
            }
        } else if lin_seg.a.y == lin_seg.b.y {
            let y = lin_seg.a.y;
            for x in
                std::cmp::min(lin_seg.a.x, lin_seg.b.x)..=std::cmp::max(lin_seg.a.x, lin_seg.b.x)
            {
                diagram[y as usize][x as usize] += 1;
                if diagram[y as usize][x as usize] == THRESHOLD {
                    danger_counter += 1;
                }
            }
        }
    }
    // println!("{:#?}", diagram);
    print_diagram(&diagram);
    println!(">>> Dangerous areas: {}", danger_counter);
}

fn part_two(input: &String) {
    const THRESHOLD: u8 = 2u8;
    println!(
        "==== PART TWO: FINDING DANGEROUS DIAGONAL AREAS WITH DANGER THRESHOLD {}",
        THRESHOLD
    );
    let (max_x, max_y, lin_segs) = find_max_values(&input);
    let mut diagram = vec![vec![0u8; (max_x + 1) as usize]; (max_y + 1) as usize];
    let mut danger_counter = 0;
    for lin_seg in lin_segs {
        if lin_seg.a.x == lin_seg.b.x {
            let x = lin_seg.a.x;
            for y in
                std::cmp::min(lin_seg.a.y, lin_seg.b.y)..=std::cmp::max(lin_seg.a.y, lin_seg.b.y)
            {
                diagram[y as usize][x as usize] += 1;
                if diagram[y as usize][x as usize] == THRESHOLD {
                    danger_counter += 1;
                }
            }
        } else if lin_seg.a.y == lin_seg.b.y {
            let y = lin_seg.a.y;
            for x in
                std::cmp::min(lin_seg.a.x, lin_seg.b.x)..=std::cmp::max(lin_seg.a.x, lin_seg.b.x)
            {
                diagram[y as usize][x as usize] += 1;
                if diagram[y as usize][x as usize] == THRESHOLD {
                    danger_counter += 1;
                }
            }
        } else {
            let mut dx = 1;
            if lin_seg.a.x > lin_seg.b.x {
                dx = -1;
            }
            let mut dy = 1;
            if lin_seg.a.y > lin_seg.b.y {
                dy = -1;
            }
            let count = lin_seg.a.x as i16 - lin_seg.b.x as i16;
            let mut x = lin_seg.a.x as i16;
            let mut y = lin_seg.a.y as i16;
            for _ in 0..(i16::abs(count) + 1) as usize {
                diagram[y as usize][x as usize] += 1;
                if diagram[y as usize][x as usize] == THRESHOLD {
                    danger_counter += 1;
                };
                x = x + dx;
                y = y + dy;
            }
        }
    }
    // println!("{:#?}", diagram);
    print_diagram(&diagram);
    println!(">>> Dangerous areas: {}", danger_counter);
}

fn find_max_values(input: &String) -> (u16, u16, Vec<LineSeg>) {
    let mut max_x = 0u16;
    let mut max_y = 0u16;

    let mut lin_segs = Vec::new();

    for line in input.lines() {
        // println!("line {}", line);
        let (c1, c2) = parse_coords(line);
        max_x = std::cmp::max(max_x, std::cmp::max(c1.x, c2.x));
        max_y = std::cmp::max(max_y, std::cmp::max(c1.y, c2.y));
        lin_segs.push(LineSeg { a: c1, b: c2 });
    }
    // println!("linsegs: {:?}", lin_segs);

    (max_x, max_y, lin_segs)
}

// fn parse_line(line: std::str::Lines, diagram: &mut Vec<Vec<u8>>) {}

fn parse_coords(line: &str) -> (Coord, Coord) {
    let coords: Vec<&str> = line.split(" -> ").collect();
    let c1 = string_to_coord(coords[0]);
    let c2 = string_to_coord(coords[1]);
    (c1, c2)
}

fn string_to_coord(nums_string: &str) -> Coord {
    let nums: Vec<u16> = nums_string
        .split(",")
        .map(|num_string| num_string.parse::<u16>().unwrap())
        .collect();
    Coord {
        x: nums[0],
        y: nums[1],
    }
}

#[derive(Debug)]
struct LineSeg {
    a: Coord,
    b: Coord,
}

#[derive(Debug)]
struct Coord {
    x: u16,
    y: u16,
}

type Diagram = Vec<Vec<u8>>;
fn print_diagram(dia: &Diagram) {
    for line in dia {
        for num in line {
            if num == &0u8 {
                print!(".");
            } else {
                print!("{}", num);
            }
        }
        print!("\n");
    }
}

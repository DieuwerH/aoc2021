use std::fs;

fn main() {
    let input = read_input(Some(false));
    // part_one(&input);
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

fn parse_input(input: &String) -> Vec<Vec<u8>> {
    let mut res = Vec::new();

    for line in input.lines() {
        // let heights = line.split_terminator("");
        // println!("H {:?}", heights);
        let heights = line
            .split_terminator("")
            .skip(1)
            .map(|ns| ns.parse::<u8>().unwrap())
            .collect::<Vec<u8>>();
        res.push(heights);
    }

    res
}

type HM = Vec<Vec<u8>>;

fn print_height_map(hm: &HM) {
    for line in hm {
        for h in line {
            print!("{} ", h);
        }
        print!("\n");
    }
}

fn find_neighbor_indices(loc: &Coord, max_x: usize, max_y: usize) -> Vec<Coord> {
    let mut res = Vec::new();

    if loc.x > 0 {
        res.push(Coord {
            x: loc.x - 1,
            y: loc.y,
        })
    }
    if loc.x < max_x {
        res.push(Coord {
            x: loc.x + 1,
            y: loc.y,
        })
    }
    if loc.y > 0 {
        res.push(Coord {
            x: loc.x,
            y: loc.y - 1,
        })
    }
    if loc.y < max_y {
        res.push(Coord {
            x: loc.x,
            y: loc.y + 1,
        })
    }

    res
}

fn nbrs_hm(loc: &Coord, hm: &HM) -> Vec<Coord> {
    let max_x = hm[0].len() - 1;
    let max_y = hm.len() - 1;
    find_neighbor_indices(loc, max_x, max_y)
}

fn part_one(input: &String) {
    let hm = parse_input(&input);
    print_height_map(&hm);
    let max_x = hm[0].len() - 1;
    let max_y = hm.len() - 1;
    let mut risk_level: u32 = 0;
    for y in 0..hm.len() {
        for x in 0..hm[0].len() {
            let loc = Coord { x, y };
            let nbrs = find_neighbor_indices(&loc, max_x, max_y);
            let slf = hm[y][x];
            let mut low_point = true;
            for nbr in nbrs {
                if hm[nbr.y][nbr.x] <= slf {
                    low_point = false;
                    break;
                }
            }
            if low_point {
                risk_level += (slf + 1) as u32;
            }
        }
    }
    println!("Risk level: {}", risk_level);
}
fn part_two(input: &String) {
    let hm = parse_input(&input);
    let mut checked = vec![vec![false; hm[0].len()]; hm.len()];
    print_height_map(&hm);
    let max_x = hm[0].len() - 1;
    let max_y = hm.len() - 1;
    let mut bassin_sizes = Vec::new();
    for y in 0..hm.len() {
        for x in 0..hm[0].len() {
            let loc = Coord { x, y };
            let nbrs = find_neighbor_indices(&loc, max_x, max_y);
            let slf = hm[y][x];
            let mut low_point = true;
            for nbr in nbrs {
                if hm[nbr.y][nbr.x] <= slf {
                    low_point = false;
                    break;
                }
            }
            if low_point {
                let bassin = find_basin(&hm, &loc, &mut checked);
                bassin_sizes.push(bassin.len());
                // println!("Size: {}", bassin.len());
            }
        }
    }
    // println!("Risk level: {}", risk_level);
    bassin_sizes.sort();
    bassin_sizes.reverse();
    let result = bassin_sizes[0] * bassin_sizes[1] * bassin_sizes[2];
    println!("Res: {}", result);
}

fn find_basin(hm: &HM, loc: &Coord, checked: &mut Vec<Vec<bool>>) -> Vec<Coord> {
    let nbrs = nbrs_hm(loc, hm);
    let slf = get(hm, loc);
    println!("Finding bassin at ({},{}) - {}", loc.x, loc.y, slf);
    let mut res = Vec::new();
    if slf == 9 {
        return res;
    } else {
        res.push(loc.clone());
        checked[loc.y][loc.x] = true;
    }
    for nb in nbrs {
        if checked[nb.y][nb.x] {
            continue;
        }
        let nb_val = get(hm, &nb);
        if nb_val >= slf {
            let found = find_basin(hm, &nb, checked);
            // println!("Found {:?}", found);
            for f in found {
                if !res.contains(&f) {
                    res.push(f);
                }
            }
        }
    }
    res
}

fn get(hm: &HM, location: &Coord) -> u8 {
    hm[location.y][location.x]
}

#[derive(Clone, PartialEq)]
struct Coord {
    x: usize,
    y: usize,
}

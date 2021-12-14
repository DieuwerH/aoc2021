use std::fs;

fn main() {
    let input = read_input(true);
    let input = read_input(false);
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
    println!("Input: \n{}", input);
    let mut coords = Vec::new();
    let mut mx = 0;
    let mut my = 0;
    let parts: Vec<&str> = input.split("\n\n").collect();
    for line in parts[0].lines() {
        let coord = line
            .split(",")
            .map(|l| l.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let coord = Coord {
            x: coord[0],
            y: coord[1],
        };
        if coord.x > mx {
            mx = coord.x;
        }
        if coord.y > my {
            my = coord.y;
        }
        coords.push(coord);
    }
    let mut paper = vec![vec!["."; my as usize + 1]; mx as usize + 1];

    for coord in coords {
        paper[coord.x as usize][coord.y as usize] = "#";
    }

    print_paper(&paper, Some(0));

    let mut fold_counter = 0;
    for line in parts[1].lines() {
        fold_counter +=1;
        let instruction: Vec<&str> = line.split("=").collect();
        let axis = instruction[0];
        let linenr = instruction[1].parse::<usize>().unwrap();

        let xstart1 = 0;
        let mut xend1 = paper.len();
        let ystart1 = 0;
        let mut yend1 = paper[0].len();
        let mut xstart2 = 0;
        let xend2 = paper.len();
        let mut ystart2 = 0;
        let yend2 = paper[0].len();
        let mut res: Vec<Vec<&str>>;
        if axis == "y" {
            yend1 = linenr as usize;
            ystart2 = linenr as usize + 1;
            res = vec![vec!["."; linenr as usize]; paper.len()];
            // for x in 0..paper.len() {
            //     for y in 
            // }
        } else {
            xend1 = linenr as usize;
            xstart2 = linenr as usize + 1;
            res = vec![vec!["."; paper[0].len()]; linenr as usize];
        }

        for x in xstart1..xend1 {
            for y in ystart1..yend1 {
                res[x][y] = paper[x][y];
            }
        }
        for x in xstart2..xend2 {
            for y in ystart2..yend2 {
                let mut x_ = x as usize;
                let mut y_ = y as usize;
                if axis == "y" {
                    y_ = linenr - (y_ - linenr);
                } else {
                    x_ = linenr - (x_ - linenr);
                }
                if paper[x][y] == "#" {
                    res[x_ as usize][y_ as usize] = paper[x][y];

                }
            }
        }
        paper = res;
        print_paper(&paper, Some(fold_counter));
    }
}

struct Coord {
    x: usize,
    y: usize,
}

fn print_paper(paper: &Vec<Vec<&str>>, fold_num: Option<u8>) {
    if let Some(num) = fold_num {
        println!(" === #Folds {} ===", num);
    }
    let mut counter = 0;
    for y in 0..paper[0].len() {
        for x in 0..paper.len() {
            let chr = paper[x][y];
            if chr == "#" {
                counter += 1;
                print!("█");
            } else {
                print!(" ")
            }
        }
        print!("\n");
    }
    println!(" === #Dots {} === \n", counter);
}

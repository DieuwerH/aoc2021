use std::collections::HashMap;
use std::fs;

fn main() {
    let input = read_input(false);
    // let input = read_input(false);
    // part_one(&input);
    part_two(&input);
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
    println!("input:\n{}", input);
    let mut lines = input.lines();
    let enhance_algo = lines.next().unwrap();
    let mut enhance_map = HashMap::new();
    for (i, chr) in enhance_algo.chars().enumerate() {
        enhance_map.insert(i, chr);
    }
    lines.next();
    let mut img = Vec::new();
    for line in lines {
        let mut img_line = Vec::new();
        for chr in line.chars() {
            img_line.push(chr);
        }
        img.push(img_line);
    }
    println!("Input image:");
    print_image(&img, true);
    let enhanced = enhance(img, &enhance_map, '.');
    println!("Enhanced image:");
    print_image(&enhanced, true);
    let en2 = enhance(enhanced, &enhance_map, '#');
    println!("EnH2");
    print_image(&en2, false);
    let mut counter = 0;
    for line in &en2 {
        for chr in line {
            if chr == &'#' {
                counter += 1;
            }
        }
    }
    println!("{} pixels are lit", counter);
}

fn part_two(input: &String) {
    let mut lines = input.lines();
    let enhance_algo = lines.next().unwrap();
    let mut enhance_map = HashMap::new();
    for (i, chr) in enhance_algo.chars().enumerate() {
        enhance_map.insert(i, chr);
    }
    lines.next();
    let mut img = Vec::new();
    for line in lines {
        let mut img_line = Vec::new();
        for chr in line.chars() {
            img_line.push(chr);
        }
        img.push(img_line);
    }

    let mut enh_img = img;
    let mut universe = '.';
    let mut alternate_universe = '#'; //enhance_map[&(enhance_algo.chars().count()-1)];
    println!("Universe: {}, alternate universe: {}", &universe, &alternate_universe);
    for _ in 0..50 {
        enh_img = enhance(enh_img, &enhance_map, universe);
        let tmp = universe;
        universe = alternate_universe;
        alternate_universe = tmp;
    }

   
    // print_image(&en2, false);
    let mut counter = 0;
    for line in &enh_img {
        for chr in line {
            if chr == &'#' {
                counter += 1;
            }
        }
    }
    println!("{} pixels are lit", counter); 
}

fn print_image(img: &Vec<Vec<char>>, fancy: bool) {
    for row in img {
        for chr in row {
            let mut to_print = chr;
            if fancy {
                to_print = &'█';
                if chr != &'#' {
                    to_print = &' ';
                }
            }
            print!("{}", to_print);
        }
        println!();
    }
}

fn enhance(
    img: Vec<Vec<char>>,
    enhance_map: &HashMap<usize, char>,
    out_of_reach: char,
) -> Vec<Vec<char>> {
    let mut res = Vec::new();

    for y in -1..=img.len() as isize {
        let mut row = Vec::new();
        for x in -1..=img[0].len() as isize {
            let mut bin_string = Vec::new();
            let xmin = x - 1;
            let xmax = x + 1;
            let ymin = y - 1;
            let ymax = y + 1;
            for ycur in ymin..=ymax {
                for xcur in xmin..=xmax {
                    // println!("{},{}", xcur, ycur);
                    if xcur < 0 || ycur < 0 {
                        bin_string.push(out_of_reach);
                    } else if xcur >= (img[0].len() as isize) || ycur >= (img.len() as isize) {
                        bin_string.push(out_of_reach);
                    } else {
                        bin_string.push(img[ycur as usize][xcur as usize])
                    }
                }
            }
            let chr_index = binary_to_dec(&bin_string);
            row.push(enhance_map[&chr_index]);
        }
        res.push(row);
    }

    res
}

fn binary_to_dec(bin_string: &Vec<char>) -> usize {
    // println!("Bin decoder input: {:?}", bin_string);
    let max_pow = bin_string.len() - 1;
    let mut res = 0usize;
    for (i, chr) in bin_string.iter().enumerate() {
        if chr == &'#' {
            let curpow = (max_pow - i) as u32;
            res += 2usize.pow(curpow);
        }
    }
    // println!("Found number:{}", res);
    res
}

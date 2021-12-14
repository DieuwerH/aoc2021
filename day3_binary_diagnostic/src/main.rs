use std::fs;

fn main() {
    println!("PART 1");
    part_one();
    println!("PART 2");
    part_two();
}

fn part_one() {
    // let filename = "src/example.txt";
    let filename = "src/input.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let bits: Vec<Vec<&str>> = contents.lines().map(|l| l.split("").collect()).collect();

    let mut g: Vec<char> = Vec::new();
    let mut e: Vec<char> = Vec::new();

    for i in 1..bits[0].len() - 1 {
        let mut count = 0;
        for j in 0..bits.len() {
            if bits[j][i] == "1" {
                count += 1;
            } else {
                count -= 1;
            }
        }
        if count > 0 {
            g.push('1');
            e.push('0');
        } else {
            g.push('0');
            e.push('1');
        }
    }

    let gamma: String = g.into_iter().collect();
    let epsilon: String = e.into_iter().collect();

    let num_gam = isize::from_str_radix(&gamma, 2).unwrap();
    let num_eps = isize::from_str_radix(&epsilon, 2).unwrap();

    println!("Gamma:\t\t{}\t{}", gamma, num_gam);
    println!("Epsilon:\t{}\t{}", epsilon, num_eps);
    println!("Multiplied: {}", num_gam * num_eps);
}

fn part_two() {
    // let filename = "src/example.txt";
    let filename = "src/input.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let bits: Vec<Vec<&str>> = contents
        .lines()
        .map(|l| l.split_terminator("").skip(1).collect())
        .collect();

    let mut mf_indices: Vec<usize> = (0..bits.len()).map(|x| x).collect();
    let mut lf_indices: Vec<usize> = (0..bits.len()).map(|x| x).collect();
    let mut col = 0;
    while mf_indices.len() > 1 {
        mf_indices = find_most_frequents(&mf_indices, &bits, col, true);
        println!("Indices: {:?}", &mf_indices);
        col = (col + 1) % 12;
    }

    let mut lf_col = 0;
    while lf_indices.len() > 1 {
        lf_indices = find_most_frequents(&lf_indices, &bits, lf_col, false);
        println!("Indices: {:?}", &lf_indices);
        lf_col = (lf_col + 1) % 12;
    }

    let oxy_bin: String = bits[mf_indices[0]].clone().into_iter().collect();
    let oxy_num = isize::from_str_radix(&oxy_bin, 2).unwrap();
    let co2_bin: String = bits[lf_indices[0]].clone().into_iter().collect();
    let co2_num = isize::from_str_radix(&co2_bin, 2).unwrap();
    println!("Oxy:\t{:?}\t{:?}", oxy_bin, oxy_num);
    println!("CO2:\t{:?}\t{:?}", co2_bin, co2_num);
    let life_support_rating = oxy_num * co2_num;
    println!("Life support rating: {}", life_support_rating);
}

fn find_most_frequents(
    indices: &Vec<usize>,
    bits: &Vec<Vec<&str>>,
    col: usize,
    most: bool,
) -> Vec<usize> {
    let mut count = 0;
    let mut candis_1 = Vec::new();
    let mut candis_0 = Vec::new();
    for i in 0..indices.len() {
        let row = indices[i];
        let cur = bits[row][col];
        if cur == "1" {
            candis_1.push(row);
            count += 1;
        } else {
            candis_0.push(row);
            count -= 1;
        }
    }

    if most {
        if count >= 0 {
            return candis_1;
        }
        candis_0
    } else {
        if count >= 0 {
            return candis_0;
        }
        candis_1
    }
}

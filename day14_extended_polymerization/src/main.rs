use std::collections::HashMap;
use std::fs;

fn main() {
    let input = read_input(true);
    let input = read_input(false);
    part_one(&input);
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
    println!("Input:\n{}", input);
    let split = input.split("\n\n").collect::<Vec<&str>>();
    let mut poly = split[0].chars().collect::<Vec<char>>();
    let maps = split[1];
    let mut mapping: HashMap<&str, char> = HashMap::new();
    for map in maps.lines() {
        let spl = map.split(" -> ").collect::<Vec<&str>>();
        let chr = spl[1].chars().collect::<Vec<char>>()[0];
        mapping.insert(spl[0], chr);
    }

    let steps = 10;
    for i in 0..steps {
        let mut res = String::new();
        for i in 0..poly.len() - 1 {
            let mut part = String::new();
            part.push(poly[i]);
            part.push(poly[i + 1]);
            res.push(poly[i]);
            res.push(*mapping.get(&part as &str).unwrap())
        }
        res.push(poly[poly.len() - 1]);
        println!("After step {}: {}", i + 1, res.len());
        poly = res.chars().collect::<Vec<char>>();
    }

    let mut chr_count = HashMap::new();

    for chr in poly {
        let count: u32;
        if chr_count.contains_key(&chr) {
            count = *chr_count.get(&chr).unwrap() + 1;
        } else {
            count = 1;
        }
        chr_count.insert(chr, count);
    }
    println!("{:?}", chr_count);

    let mut max = 0u32;
    let mut maxchr = '-';
    let mut min = u32::max_value();
    let mut minchr = '-';
    for key in chr_count.keys() {
        let val = chr_count.get(key).unwrap();
        if val < &min {
            min = *val;
            minchr = *key;
        }
        if val > &max {
            max = *val;
            maxchr = *key;
        }
    }
    println!("Max char: {}, with {} occurences", maxchr, max);
    println!("Min char: {}, with {} occurences", minchr, min);
    println!("Score: {}", max - min)
}

fn part_two(input: &String) {
    println!("=== Part Two ===");
    println!("Input: {}", &input);
    let split = input.split("\n\n").collect::<Vec<&str>>();
    let mut poly = split[0].chars().collect::<Vec<char>>();
    let maps = split[1];
    let mut mapping: HashMap<&str, char> = HashMap::new();
    let mut tracker: HashMap<&str, u64> = HashMap::new();
    let mut updates: HashMap<&str, u64> = HashMap::new();
    for map in maps.lines() {
        let spl = map.split(" -> ").collect::<Vec<&str>>();
        let chr = spl[1].chars().collect::<Vec<char>>()[0];
        mapping.insert(spl[0], chr);
        tracker.insert(spl[0], 0u64);
        updates.insert(spl[0], 0u64);
    }
    let mut chr_count = HashMap::new();

    for chr in &poly {
        *chr_count.entry(*chr).or_insert(0u64) += 1;
    }

    // Initial setup of tracker
    for i in 0..poly.len() - 1 {
        let mut part = String::new();
        part.push(poly[i]);
        part.push(poly[i + 1]);
        println!("{}:", part);
        let occurence = tracker.get_mut(&part as &str).unwrap();
        *occurence += 1;
    }

    println!("Tracker:\n{:?}", tracker);

    let steps = 40;
    for _ in 0..steps {
        for key in tracker.clone().keys() {
            let cur_chars = key.chars().collect::<Vec<char>>();
            let val = tracker[key]; // Current pair of chars
            let mapped = mapping.get(key).unwrap(); // Char it is mapped to
            let mut new1 = String::new(); // New 1 is first char of current + mapped
            new1.push(cur_chars[0]);
            new1.push(*mapped);
            let mut new2 = String::new(); // New 2 is mapped + 2nd char of current
            new2.push(*mapped);
            new2.push(cur_chars[1]);

            let count1 = updates.get_mut(&new1 as &str).unwrap();
            *count1 += val;
            let count2 = updates.get_mut(&new2 as &str).unwrap();
            *count2 += val;
            tracker.insert(key, 0);
            *chr_count.entry(*mapped).or_insert(0) += val as u64;
        }
        for key in updates.clone().keys() {
            let val = updates[key];
            tracker.insert(key, val);
            updates.insert(key, 0);
        }
        // println!("Tracker:\n{:?}", tracker);
    }

    // let mut chr_count = HashMap::new();

    // for chr in poly {
    //     let count: u32;
    //     if chr_count.contains_key(&chr) {
    //         count = *chr_count.get(&chr).unwrap() + 1;
    //     } else {
    //         count = 1;
    //     }
    //     chr_count.insert(chr, count);
    // }
    println!("{:?}", chr_count);

    let mut max = 0u64;
    let mut maxchr = '-';
    let mut min = u64::max_value();
    let mut minchr = '-';
    for key in chr_count.keys() {
        let val = chr_count.get(key).unwrap();
        if val < &min {
            min = *val;
            minchr = *key;
        }
        if val > &max {
            max = *val;
            maxchr = *key;
        }
    }
    println!("Max char: {}, with {} occurences", maxchr, max);
    println!("Min char: {}, with {} occurences", minchr, min);
    println!("Score: {}", max - min)
}

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
    // let line = input.split(" | ").collect::<Vec<&str>>();
    // let patterns = line[0];
    // let out = line[1];
    // println!("Patterns: {}", patterns);

    let letters = vec![
        "abcefg", "cf", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
    ];
    let mut freq_map = vec![Vec::new(); 8];
    for i in 0..letters.len() {
        let letter = letters[i];
        for chr in letter.chars() {
            if !freq_map[letter.chars().count() as usize].contains(&chr) {
                freq_map[letter.chars().count() as usize].push(chr);
            }
        }
    }
    println!("Freq map: {:?}", freq_map);

    // let mut options = vec![Vec::new(); 7];
    // for pat in patterns.split_whitespace() {
    //     let cur_pat = pat;
    //     for chr in pat.chars() {
    //         options[(chr as u8 - 97) as usize].push(freq_map[cur_pat.len()].clone());
    //         // println!("Char: {} - {:?}", chr, chr as u8);
    //     }
    // }
    // println!("Options: {:?}", options);

    // let mut sorted_pats: Vec<&str> = patterns.split_whitespace().map(|letter| letter).collect();
    // sorted_pats.sort_by(|a, b| a.chars().count().cmp(&b.chars().count()));
    // println!("sorted {:?}", sorted_pats);

    // let mut assigned_counter = [0u8; 7];
    // let mut assigned = ['-'; 7];
    // let mut assigned_inv = ['-'; 7];
    // let mut assinged_letters = Vec::new();
    // let mut tmp = vec![Vec::new(); 7];
    // for sp in sorted_pats {
    //     println!("Checking part: {}", sp);
    //     let len = sp.len();

    //     for chr in sp.chars() {
    //         let chr_index = (chr as u8 - 97) as usize;
    //         println!("  checking {} - options: {:?}", chr, freq_map[len].clone());
    //         if tmp[chr_index].len() >= 2 {
    //             println!("    {} already has two options assigned", chr);
    //         } else {
    //             for letter_option in freq_map[len].clone() {
    //                 // println!("  {} can be {}", chr, letter_option);
    //                 if assinged_letters.contains(&letter_option) {
    //                     println!("    {} has already been assigned", letter_option);
    //                     continue;
    //                 }
    //                 let option_index = (letter_option as u8 - 97) as usize;
    //                 if assigned[chr_index] == '-' {
    //                     // if assigned_counter[option_index] < 2 && tmp[chr_index].len() < 2 {
    //                     println!(
    //                         "\t{}, ({}) has not yet been assigned",
    //                         letter_option, option_index
    //                     );
    //                     assigned_counter[option_index] += 1;
    //                     tmp[chr_index].push(letter_option);
    //                     // } else {
    //                     //     println!("\t{} is not available for assignment", letter_option);
    //                     // }
    //                 } else {
    //                     println!(
    //                         "{}, ({}) HAS ALREADY BEEN assigned",
    //                         letter_option, option_index
    //                     );
    //                 }
    //             }
    //         }
    //         println!();
    //     }
    //     for (i, opts) in tmp.iter().enumerate() {
    //         // println!("Opts {:?}", opts);
    //         if assigned[i] == '-' {
    //             if opts.len() == 1 {
    //                 let chr = opts[0];
    //                 let char_index = (chr as u8 - 97) as usize;
    //                 let index_char = ((i + 97) as u8) as char;
    //                 assigned_inv[char_index] = index_char;
    //                 assigned[i] = chr;
    //                 assinged_letters.push(chr);
    //                 println!("Assigned: {:?}", assigned);
    //             }
    //         }
    //     }
    //     print_options(&tmp);
    //     println!("===");

    // println!("{} can be {:?}", sp, freq_map[len]);
    // for chr_option in freq_map[len] {
    //     let index = (chr as u8 - 97) as usize;
    //     if assigned[index] <= 2 {
    //         assigned[index] += 1;
    //     }
    //     // let opts = freq_map[sp.len()].clone();
    //     // // println!("Options for {}: {:?}", chr, opts);
    //     // for o_chr in opts {
    //     //     tmp[(chr as u8 - 97) as usize].push(o_chr);
    //     // }
    // }
    // }
    // print_options(&tmp);

    let mut counter = 0;

    for line in input.lines() {
        let all = line.split(" | ").collect::<Vec<&str>>();
        let left = all[0];
        let right = all[1];

        for letters in right.split_whitespace() {
            // println!("Letters: {}", letters);
            let len = letters.len();
            if len == 2 || len == 3 || len == 4 || len == 7 {
                counter += 1;
            }
        }
    }
    println!("Count: {}", counter);
}

fn part_two(input: &String) {
    // let input =
    // "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
    let mut sum: u32 = 0;
    for line in input.lines() {
        let all = line.split(" | ").collect::<Vec<&str>>();
        let left = all[0];
        let right = all[1];
        let mut segments: Vec<&str> = left.split_whitespace().collect();
        segments.sort_by(|a, b| a.chars().count().cmp(&b.chars().count()));
        // println!("Segments: {:?}", segments);

        // for seg in &segments {
        //     println!("{} \t len: {}", seg, seg.len());
        // }

        let one = sort_string(segments[0]);
        let one_chars = segments[0].chars().collect::<Vec<char>>();
        let seven = sort_string(segments[1]);
        let mut a = '-';
        for chr in seven.chars() {
            if !one.contains(chr) {
                a = chr;
                break;
            }
        }
        let four = sort_string(segments[2]);
        let four_ = four
            .chars()
            .filter(|f| !one_chars.contains(f))
            .collect::<Vec<char>>();
        let mut two = String::from("-");
        let mut three = String::from("-");
        let mut five = String::from("-");

        for i in 3..6 {
            let seg = sort_string(segments[i]);
            if seg.contains(one_chars[0]) && seg.contains(one_chars[1]) {
                three = seg;
            } else if seg.contains(four_[0]) && seg.contains(four_[1]) {
                five = seg;
            } else {
                two = seg;
            }
        }

        let mut zero = String::from("-");
        let mut six = String::from("-");
        let mut nine = String::from("-");

        for i in 6..9 {
            let seg = sort_string(segments[i]);
            if seg.contains(one_chars[0]) && seg.contains(one_chars[1]) {
                if seg.contains(four_[0]) && seg.contains(four_[1]) {
                    nine = seg;
                } else {
                    zero = seg;
                }
            } else {
                six = seg;
            }
        }

        // println!("Right: {}", right);

        let eight = sort_string(segments[9]);
        let res = [zero, one, two, three, four, five, six, seven, eight, nine];
        for (i, r) in res.iter().enumerate() {
            // println!("{} : {}", i, r);
        }
        let orders: [u32; 4] = [1000, 100, 10, 1];
        let mut outnum: u32 = 0;
        for (order, num) in right.split_whitespace().enumerate() {
            let num = sort_string(num);
            for (i, r) in res.iter().enumerate() {
                if r == &num {
                    outnum += orders[order] * i as u32;
                }
            }
        }
        println!("Output: {}", outnum);
        sum += outnum;
    }
    println!("Sum: {}", sum);
}

// fn print_options(options: &Vec<Vec<char>>) {
//     let letters = "abcdefg";

//     for (i, chr) in letters.enumerate() {
//         println!("{} - {:?}", chr, options[i]);
//     }
// }

fn sort_string(inp: &str) -> String {
    let mut chars: Vec<char> = inp.chars().collect();
    chars.sort();
    chars.into_iter().collect()
}

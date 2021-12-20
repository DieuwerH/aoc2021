use std::fs;

fn main() {
    let input = read_input(true);
    // let input = read_input(false);
    part_one(&input);
    // part_two(&input);
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
    // println!("Input:\n{}", input);
    let scanners = parse_scanners(input);

    let cur_scan = &scanners[0];
    for i in 1..scanners.len() {
        let other = &scanners[i];
        let common = cur_scan.compare_dists(other);
        if common >= 132 {
            
        }
    }

    for sca in &scanners {
        println!("{} sees {} beacons", &sca.name, &sca.beacons.len());
        for scb in &scanners {
            if sca != scb {
                let shared = sca.compare_dists(&scb);
                println!("{} and {} share {} distances", &sca.name, &scb.name, shared);
                
            }
        }
    }


    // println!("{}\n{:?}\n{:?}", &scanners[0].name, &scanners[0].beacons, &scanners[0].dists)
}

fn parse_scanners(input: &String) -> Vec<Scanner> {
    let splits = input.split("\n\n");
    let scanners = splits.map(|s| Scanner::new(s)).collect::<Vec<Scanner>>();
    scanners
}

#[derive(PartialEq)]
struct Scanner {
    name: String,
    beacons: Vec<Coord>,
    dists: Vec<f32>,
}
impl Scanner {
    fn new(in_string: &str) -> Self {
        let mut beacons = Vec::new();

        let mut lines = in_string.lines();
        let header = lines.next().unwrap();
        let header= header.replace("-", "");
        let name = String::from(header);

        for line in lines {
            beacons.push(Coord::new_from_string(line));
        }

        let mut sc = Scanner {
            name,
            beacons,
            dists: Vec::new(),
        };
        sc.beacon_dists();
        sc
    }
    fn beacon_dists(&mut self) {
        let mut res = Vec::new();

        for beacon in &self.beacons {
            for other in &self.beacons {
                let dist = beacon.dist(other);
                res.push(dist);
            }
        }

        self.dists = res;
    }
    fn compare_dists(&self, other: &Scanner) -> usize {
        let mut count = 0;
        for i in 0..self.dists.len() {
            let dist_self = &self.dists[i];
            if dist_self == &0.0 {
                continue;
            }
            for j in 0..other.dists.len() {
                let dist_other = &other.dists[j];
                if dist_self == dist_other {
                    count += 1;
                }
            } 
        }
        // for dist_self in &self.dists {
        //     for dist_other in &other.dists {
        //         if dist_self != &0.0 && dist_self == dist_other {
        //             count += 1;
        //         }
        //     }
        // }
        count / 2
    }
}

#[derive(Debug, PartialEq)]
struct Coord {
    a: f32,
    b: f32,
    c: f32,
}
impl Coord {
    fn new(a: f32, b: f32, c: f32) -> Self {
        Self { a, b, c }
    }
    fn new_from_string(in_string: &str) -> Self {
        let nums = in_string
            .split(",")
            .map(|ns| ns.parse::<f32>().unwrap())
            .collect::<Vec<f32>>();
        Coord::new(nums[0], nums[1], nums[2])
    }

    fn dist(&self, other: &Coord) -> f32 {
        let a2 = (self.a - other.a).powf(2.0);
        let b2 = (self.b - other.b).powf(2.0);
        let c2 = (self.c - other.c).powf(2.0);
        (a2 + b2 + c2).powf(0.5)
    }
}

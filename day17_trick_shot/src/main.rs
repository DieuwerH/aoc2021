use std::fs;

fn main() {
    let input = read_input(false);
    // let input = read_input(false);
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
    println!("Input:\n{}", input);
    let split = input.split(": ").collect::<Vec<&str>>();
    let split = split[1].split(", ").collect::<Vec<&str>>();
    let (xmin, xmax) = coord_extractor(split[0]);
    println!("Xmin: {}, Xmax: {}", xmin, xmax);
    let (ymax, ymin) = coord_extractor(split[1]);
    println!("Ymin: {}, Ymax: {}", ymin, ymax);

    for y in (ymax..0).rev() {
        // println!("Cur y: {}", y);
        for x in 0..xmax {
            // println!("Cur x: {}", x);
            let mut chr = ".";
            if x >= xmin && y <= ymin {
                chr = "T";
            }
            print!("{}", chr);
        }
        println!();
    }

    let mut state = State::new(7, 2, xmin, xmax, ymin, ymax);
    state.brute_force();
}

struct Target {
    minx: i64,
    maxx: i64,
    miny: i64,
    maxy: i64,
}

impl Target {
    fn overshot(&self, x: i64, y: i64) -> bool {
        x > self.maxx || y < self.maxy
    }
    fn on_target(&self, x: i64, y: i64) -> bool {
        x >= self.minx && x <= self.maxx && y <= self.miny && y >= self.maxy
    }
}

struct State {
    x: i64,
    y: i64,
    vx: i64,
    vy: i64,
    t: Target,
    maxy: i64,
}
impl State {
    fn new(vx: i64, vy: i64, minx: i64, maxx: i64, miny: i64, maxy: i64) -> Self {
        Self {
            x: 0,
            y: 0,
            vx,
            vy,
            t: Target {
                minx,
                maxx,
                miny,
                maxy,
            },
            maxy: 0
        }
    }
    fn step(&mut self) {
        // println!("Velocity x: {}, y: {}", self.vx, self.vy);
        self.x += self.vx;
        self.y += self.vy;
        if self.vx > 0 {
            self.vx -= 1;
        } else if self.vx < 0 {
            self.vx += 1
        }
        self.vy -= 1;
        if self.y > self.maxy {
            self.maxy = self.y
        }
    }
    fn overshot(&self) -> bool {
        self.t.overshot(self.x, self.y)
    }
    fn on_target(&self) -> bool {
        self.t.on_target(self.x, self.y)
    }
    fn simulate(&mut self) -> bool {
        while !self.overshot() {
            self.step();
            // println!("New coords: ({},{})", self.x, self.y);
            if self.on_target() {
                // println!("Winner winner chicken dinner!");
                // println!("At coords: ({},{})", self.x, self.y);
                return true
            }
        }
        false
    }
    fn reset(&mut self) {
        self.x = 0;
        self.y = 0;
        self.vx = 0;
        self.vy = 0;
        self.maxy = 0;
    }
    fn brute_force(&mut self) {
        let mut max = 0;
        let mut winner : (i64,i64) = (0,0);

        let mut counter = 0;
        for x in -1000..1000 {
            for y in -1000..1000 {
                self.reset();
                self.vx = x;
                self.vy = y;
                if self.simulate() {
                    counter+=1;
                    println!("Velocity {},{} succeeds, (max y: {})", x, y, self.maxy);
                    if self.maxy > max {
                        max = self.maxy;
                        winner = (x, y);
                    }
                }

            }
        }
        println!("Highest y ({}) reached with velocity {},{}", max, winner.0, winner.1);
        println!("options: {}", counter);
    }
}

fn coord_extractor(coord: &str) -> (i64, i64) {
    let coord = coord.split("=").collect::<Vec<&str>>();
    let coord = coord[1].split("..").collect::<Vec<&str>>();
    let min = coord[0].parse::<i64>().unwrap();
    let max = coord[1].parse::<i64>().unwrap();

    (min, max)
}

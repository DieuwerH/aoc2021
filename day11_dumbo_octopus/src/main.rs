use ggez::event::{self, EventHandler};
// use ggez::conf::{Conf, WindowMode};
use ggez::graphics::DrawParam;
use ggez::graphics::{self, Color, Rect};
use ggez::input::mouse::MouseButton;
use ggez::{Context, ContextBuilder, GameResult};
use std::fs;

fn main() {
    let input = read_input(false);

    let (mut ctx, event_loop) = ContextBuilder::new("Dumbo Octopus", "h0nd")
        .build()
        .expect("aieee, could not create ggez context");
    let octs = parse(&input);
    let viz = Viz::new(&mut ctx, octs);
    event::run(ctx, event_loop, viz);
}

fn read_input(use_example: bool) -> String {
    let mut filename = "./src/input.txt";

    if use_example {
        filename = "./src/example.txt";
    }

    let res = fs::read_to_string(filename).unwrap();

    res
}

fn parse(input: &String) -> Vec<Vec<u8>> {
    let mut res = Vec::new();

    for line in input.lines() {
        let nums = line
            .split_terminator("")
            .skip(1)
            .map(|ns| ns.parse::<u8>().unwrap())
            .collect::<Vec<u8>>();
        res.push(nums);
    }

    res
}

type Coord = [u8; 2];
fn coord(x: usize, y: usize) -> Coord {
    [x as u8, y as u8]
}
fn nbrs(loc: &Coord, octs: Vec<Vec<u8>>) -> Vec<Coord> {
    let mut res = Vec::new();
    let x = loc[0] as isize;
    let y = loc[1] as isize;
    let mx = (octs[0].len()) as isize;
    let my = (octs.len()) as isize;

    let xops = [x - 1, x, x + 1];
    let yops = [y - 1, y, y + 1];

    for xop in xops {
        for yop in yops {
            if xop >= 0 && xop < mx && yop >= 0 && yop < my && !(xop == x && yop == y) {
                res.push(coord(xop as usize, yop as usize));
            }
        }
    }

    res
}

struct History {
    octs: Vec<Vec<u8>>,
    fcount: u32,
    scount: u32,
}

struct Viz {
    rects: Vec<Rect>,
    octs: Vec<Vec<u8>>,
    flash_counter: u32,
    step_counter: u32,
    history: Vec<History>,
}
impl Viz {
    pub fn new(_ctx: &mut Context, octs: Vec<Vec<u8>>) -> Viz {
        let mut rects = Vec::new();
        let size = 50.0;
        let hw = size - 5.0;
        for y in 0..10 {
            for x in 0..10 {
                let rect = Rect::new(x as f32 * size, y as f32 * size, hw, hw);
                rects.push(rect)
            }
        }
        Viz {
            rects,
            octs,
            flash_counter: 0,
            step_counter: 0,
            history: Vec::new(),
        }
    }

    fn flat_octs(&self) -> Vec<u8> {
        self.octs.clone().into_iter().flatten().collect()
    }

    fn step(&mut self, ctx: &mut Context) {
        // self.draw(ctx).unwrap();
        self.history.push(History {
            octs: self.octs.clone(),
            fcount: self.flash_counter,
            scount: self.step_counter,
        });
        for x in 0..10 {
            for y in 0..10 {
                self.octs[y][x] += 1;
                if self.octs[y][x] == 10 {
                    self.flash(&coord(x, y), ctx);
                }
            }
        }
        for x in 0..10 {
            for y in 0..10 {
                if self.octs[y][x] > 9 {
                    self.octs[y][x] = 0;
                }
            }
        }
        self.step_counter += 1;
    }

    fn flash(&mut self, loc: &Coord, ctx: &mut Context) {
        // println!("Flashing {} {}", loc[0], loc[1]);
        self.flash_counter += 1;
        // self.draw(ctx).unwrap();
        let neighs = nbrs(loc, self.octs.clone());
        let mut to_flash = Vec::new();
        for nb in neighs {
            let x = nb[0] as usize;
            let y = nb[1] as usize;
            self.octs[y][x] += 1;
            if self.octs[y][x] == 10 {
                to_flash.push(coord(x, y))
            }
        }
        for loc in to_flash {
            self.flash(&loc, ctx);
        }
    }

    fn step_back(&mut self) {
        let state = self.history.pop();
        if let Some(s) = state {
            self.octs = s.octs;
            self.flash_counter = s.fcount;
            self.step_counter = s.scount;
        }
    }
}
impl EventHandler for Viz {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        if self.step_counter < 370 {
            self.step(_ctx);
        }
        std::thread::sleep_ms(10);
        Ok(())
    }
    fn draw(&mut self, _ctx: &mut Context) -> GameResult<()> {
        graphics::clear(_ctx, Color::WHITE);
        for (rect, oct) in self.rects.iter().zip(self.flat_octs()) {
            let mut clr = Color::BLUE;
            if oct == 0 {
                clr = Color::YELLOW;
            }
            let r1 = graphics::Mesh::new_rectangle(_ctx, graphics::DrawMode::fill(), *rect, clr)?;
            let text = graphics::Text::new(oct.to_string());
            graphics::draw(_ctx, &r1, DrawParam::default())?;
            let prms = DrawParam::new().dest(rect.point()).offset([-10.0, -10.0]);
            // println!("{:?}", rect.point());
            graphics::draw(_ctx, &text, prms)?;
        }

        let header = graphics::TextFragment::new("STEPS:");
        let steps = graphics::TextFragment::new(self.step_counter.to_string());
        let h2 = graphics::TextFragment::new("\nFLASHES:");
        let flashes = graphics::TextFragment::new(self.flash_counter.to_string());
        let mut text = graphics::Text::new(header);
        text.add(steps);
        text.add(h2);
        text.add(flashes);
        let prms = DrawParam::new().offset([-550.0, -50.0]).color(Color::BLACK);
        // graphics::draw(_ctx, &header, prms.offset([0.0, 0.0]))?;
        graphics::draw(_ctx, &text.clone(), prms)?;
        // let rect = graphics::Rect::new(450.0, 450.0, 50.0, 50.0);
        graphics::present(_ctx)
    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
        if _button == MouseButton::Left {
            self.step(_ctx);
        }
        if _button == MouseButton::Right {
            self.step_back();
        }
        println!(
            "Flashes after {} steps: {}",
            self.step_counter, self.flash_counter
        );
    }
}

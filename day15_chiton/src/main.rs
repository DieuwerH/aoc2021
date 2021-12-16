use std::fs;

fn main() {
    let input = read_input(true);
    // let input = read_input(false);
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
    let mut board = Vec::new();
    for line in input.lines() {
        let x = line
            .split_terminator("")
            .skip(1)
            .map(|c| c.parse::<u8>().unwrap())
            .collect::<Vec<u8>>();
        board.push(x);
    }

    let mut graph = Vec::new();
    for line in &board {
        let mut node_line = Vec::new();
        for val in line {
            node_line.push(Node::new(*val));
        }
        graph.push(node_line);
    }
    let maxy = board.len();
    let maxx = board[0].len();
    for y in 0..maxy {
        for x in 0..maxx {
            let curnode = &mut graph[y][x];
            let nbrs = neigh_coords(x, y, maxx, maxy);
            for nbr in nbrs {
                let othernode = graph[nbr[1]][nbr[0]];
                curnode.add_edge(othernode)
            }
        }
    }
}
fn part_two(input: &String) {}

struct Node {
    edges: Vec<Edge>,
    value: u8,
}
impl Node {
    fn new(value: u8) -> Self {
        Self {
            edges: Vec::new(),
            value,
        }
    }
    fn add_edge(&mut self, node: Node) {
        self.edges.push(Edge::new(node));
    }
}

struct Edge {
    cost: u8,
    to: Node,
}

impl Edge {
    fn new(to: Node) -> Self {
        Self { to, cost: to.value }
    }
}

fn get<'a>(vec: Vec<Vec<Node>>, x: usize, y: usize) -> Node {
    vec[y][x]
}

fn neigh_coords(x: usize, y: usize, max_x: usize, max_y: usize) -> Vec<[usize; 2]> {
    let mut res = Vec::new();

    if x > 0 {
        res.push([x - 1, y])
    }
    if x < max_x {
        res.push([x + 1, y])
    }
    if y > 0 {
        res.push([x, y - 1])
    }
    if y < max_y {
        res.push([x, y + 1])
    }

    res
}

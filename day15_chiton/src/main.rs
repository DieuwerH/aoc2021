use petgraph::algo::dijkstra;
use petgraph::graph::{Graph, Node, NodeIndex, UnGraph};
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
    let mut board = Vec::new();
    for line in input.lines() {
        let mut x = line
            .split_terminator("")
            .skip(1)
            .map(|c| c.parse::<u8>().unwrap())
            .collect::<Vec<u8>>();
        let mut a: Vec<u8> = x.iter().map(|item| increase(item, 1)).collect();
        let mut b: Vec<u8> = x.iter().map(|item| increase(item, 2)).collect();
        let mut c: Vec<u8> = x.iter().map(|item| increase(item, 3)).collect();
        let mut d: Vec<u8> = x.iter().map(|item| increase(item, 4)).collect();
        x.append(&mut a);
        x.append(&mut b);
        x.append(&mut c);
        x.append(&mut d);
        board.push(x);
    }

    let initial_board_len = board.len();
    for k in 1..5 {
        for i in 0..initial_board_len {
            let newline = board[i]
                .iter()
                .map(|item| increase(item, k as u8))
                .collect::<Vec<u8>>();
            board.push(newline);
        }
    }

    for line in &board {
        for val in line {
            print!("{}", val);
        }
        println!();
    }

    let mut graph = Graph::<usize, usize>::new();
    let mut tracker = Vec::new();
    for line in &board {
        let mut node_line = Vec::new();
        for val in line {
            node_line.push(graph.add_node(*val as usize));
        }
        tracker.push(node_line);
    }
    let maxy = board.len() - 1;
    let maxx = board[0].len() - 1;
    for y in 0..=maxy {
        for x in 0..=maxx {
            let curnode = tracker[y][x];
            let nbrs = neigh_coords(x, y, maxx, maxy);
            for nbr in nbrs {
                let othernode = tracker[nbr[1]][nbr[0]];
                let weight = *graph.node_weight(othernode).unwrap();
                // let weight = board[nbr[1]][nbr[0]];
                graph.add_edge(curnode, othernode, weight);
            }
        }
    }

    let end_node = tracker[maxy][maxx];
    let res = dijkstra(&graph, tracker[0][0], Some(end_node), |e| *e.weight());
    // println!("To node: {:?}", tracker[maxy][maxx]);

    println!("{:#?}", res[&end_node]);
}
fn part_two(input: &String) {}

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

fn increase(inp: &u8, add: u8) -> u8 {
    let mut res = inp + add;
    if res > 9 {
        res = res - 9;
    }
    res
}

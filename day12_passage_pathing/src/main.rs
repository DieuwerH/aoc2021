use petgraph::graph::{Graph, NodeIndex, UnGraph};
use std::collections::HashMap;
use std::fs;

fn main() {
    let input = read_input(Some(false));
    part_one(&input);
    // part_two(&input);
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
    println!("PART ONE");
    println!("{}", input);

    let mut mapping = HashMap::new();
    let mut revmap = HashMap::new();
    let mut graph = Graph::new_undirected();
    for line in input.lines() {
        let names = line.trim().split("-").collect::<Vec<&str>>();
        mapping.entry(names[0]).or_insert(graph.add_node(names[0]));
        mapping.entry(names[1]).or_insert(graph.add_node(names[1]));
        revmap.entry(mapping[names[0]]).or_insert(names[0]);
        revmap.entry(mapping[names[1]]).or_insert(names[0]);
        graph.add_edge(mapping[names[0]], mapping[names[1]], 1);
    }

    let start_node = mapping["start"];
    let end_node = mapping["end"];
    println!("Start node: {:?}, end node: {:?}", start_node, end_node);
    // println!("mapping: {:?}", mapping);

    let mut all_paths = Vec::new();
    find_path(
        &graph,
        &mut vec![String::from("start")],
        start_node,
        &mut all_paths,
        true,
    );
    println!("Found {} paths", all_paths.len())

    // let mut paths = Vec::new();
    // let mut curpath = vec![start_node];
    // while true {
    //     let curnode = curpath.pop().unwrap();
    //     if curnode == end_node {
    //         break;
    //     }
    //     for n in graph.neighbors(curnode) {
    //         let name = revmap[&n];
    //         if name == name.to_lowercase() {
    //             // if !curpath.contains(&n) {
    //             //     curpath.push()
    //             // }
    //         } else {
    //         }
    //     }
    // }

    // for n in graph.neighbors(start_node) {
    //     paths.push(n);
    // }
}

fn find_path(
    graph: &UnGraph<&str, i32>,
    path: &mut Vec<String>,
    node_index: NodeIndex,
    res: &mut Vec<Vec<String>>,
    can_double: bool,
) {
    let name = graph.node_weight(node_index).unwrap();
    if name == &"end" {
        // if !res.contains(&path.clone()) {
            // println!(" .   Path to end (#{}): {:?}", res.len(), path);
            res.push(path.clone());
            return;
        // }
    }

    // print!("Neighbours: ");
    // for n in graph.neighbors(node_index) {
    // print!("{} ", graph.node_weight(n).unwrap());
    // }
    // println!();
    for n in graph.neighbors(node_index) {
        let n_name = String::from(*graph.node_weight(n).unwrap());
        // println!("At node {}, neighbour {}", name, n_name);

        // println!(" . neigh name: {}", n_name);
        if n_name == n_name.to_lowercase() {
            // println!(" .   lowercase letter");
            if path.contains(&n_name) {
                // println!(" .   letter already in path, returning");
                if n_name != "start" && can_double {
                    path.push(n_name);
                    find_path(&graph, path, n, res, false);
                    path.pop();
                } else {
                    continue;
                }
            } else {
                // println!(" .   encountered new letter, adding to path");
                path.push(n_name);
                find_path(&graph, path, n, res, can_double);
                path.pop();
            }
        } else {
            path.push(n_name);
            find_path(&graph, path, n, res, can_double);
            path.pop();
        }
    }
}

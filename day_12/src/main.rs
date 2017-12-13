use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

use std::collections::HashSet;
use std::collections::HashMap;

fn find_graph_groups(graph: HashMap<u32, Vec<u32>>) -> Vec<HashSet<u32>> {

    let mut groups: Vec<HashSet<u32 >> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    for index in graph.keys() {
        indices.push(*index);
    }

    for index in indices {
        // Check if the index belongs in a group. If not, process it.
        let mut skip = false;
        for group in groups.clone() {
            if group.contains(&index) {
                skip = true;
                break;
            }
        }

        if skip {
            continue;
        }

        let mut stack: HashSet<u32> = HashSet::new();
        stack.insert(index);
        let group = traverse_graph(index, &graph, stack);
        groups.push(group);
    }
    groups
}

fn traverse_graph(index: u32, graph: &HashMap<u32, Vec<u32>>, stack: HashSet<u32>) -> HashSet<u32> {

    let mut new_stack = stack.clone();
    for child in graph.get(&index).unwrap() {
        if !new_stack.contains(&child.clone()) {

            new_stack.insert(*child);
            let output =  traverse_graph(*child, graph, new_stack.clone());
            for value in output {
                new_stack.insert(value);
            }
        }
        
    }
    new_stack
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: String = match args.get(1)  {
        None => panic!("You must supply a file to evaluate."),
        Some(filename) => filename.clone()
    };

    println!("Processing file {:?}", filename);

    let f = File::open(filename).expect("file not found");
    let file = BufReader::new(&f);

    let mut edges: HashSet<(u32, u32)> = HashSet::new();
    

    for wrapped_line in file.lines() {
        let line = wrapped_line.unwrap();

        let line_parts = line.split_whitespace().collect::<Vec<&str>>();

        let parent: u32 = line_parts[0].to_string().parse().unwrap();
        let mut children: Vec<String> = Vec::new();
        for part in &line_parts[2..] {
            let node: u32 = part.to_string().replace(",", "").parse().unwrap();
            let edge: (u32, u32);
            edge = (parent, node);
            edges.insert(edge);
        }
    }

    let mut edge_map: HashMap<u32, Vec<u32>> = HashMap::new();
    for edge in &edges {
        // Check if the map is already present. If so, add the target to the
        // id list.

        if edge_map.contains_key(&edge.0) {
            let mut new_edge: Vec<u32> = edge_map.get(&edge.0).unwrap().clone();
            new_edge.push(edge.1);
            edge_map.insert(edge.0, new_edge);
        } else {
            let mut edge_list: Vec<u32> = Vec::new();
            edge_list.push(edge.1);
            edge_map.insert(edge.0, edge_list);
        }
    }

    println!("{:?}", edge_map);
    let mut stack: HashSet<u32> = HashSet::new();
    stack.insert(0_u32);
    let set = traverse_graph(0, &edge_map, stack);
    
    println!("Group 0 has {:?} elements.", set.len());

    let groups = find_graph_groups(edge_map);
    println!("There are {:?} groups in the graph.", groups.len());
}

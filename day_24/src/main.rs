use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

use std::collections::HashSet;
use std::collections::HashMap;

#[derive(Debug)]
struct Graph {
    edges: Vec<(u32, u32)>,
    vertices: HashMap<u32, (u32, u32)>,
    // I'm going to need to know the neighbors a lot, so we might as well cache 
    // a map of the neighbors for each vertex.
    neighbors: HashMap<u32, HashSet<u32>>
}



impl Graph {

    fn new() -> Graph {
        Graph {
            edges: Vec::new(),
            vertices: HashMap::new(),
            neighbors: HashMap::new()
        }
    }

    fn add_vertex(&mut self, vertex: (u32, u32)) {
        let vertex_count: u32 = self.vertices.clone().len() as u32;
        self.vertices.insert(vertex_count, vertex);
    }

    fn add_edge(&mut self, edge: (u32, u32)) {
        self.edges.push(edge);

        let mut vertex_neighbors: HashSet<u32> = match self.neighbors.get(&edge.0) {
            Some(set) => set.clone(), 
            None => HashSet::new()
        };
        vertex_neighbors.insert(edge.1);
        self.neighbors.insert(edge.0, vertex_neighbors.clone());

        let mut vertex_neighbors: HashSet<u32> = match self.neighbors.get(&edge.1) {
            Some(set) => set.clone(), 
            None => HashSet::new()
        };

        vertex_neighbors.insert(edge.0);
        self.neighbors.insert(edge.1, vertex_neighbors.clone());
    }

    fn bridge_strength(&self, bridge: &Vec<u32>) -> u32{
        let mut index = 0;
        let mut strength = 0;

        for element in bridge {
            let vertex = self.vertices.get(&element).unwrap();
            strength += vertex.0 + vertex.1;
        }
        strength
    }

    fn strongest_bridge(&self, search_index: u32) -> Vec<u32> {

        // 
        // let mut current_path: Vec<u32> = Vec::new();

        // let mut search_stack: Vec<u32> = Vec::new();
        // search_stack.push(search_index);
        // loop {
        //     let current_vertex = search_stack.pop().unwrap();
        //     let current_path = path.push(current_vertex);



        // }

        let mut path_candidates: Vec<Vec<u32>> = Vec::new();
        let mut path: Vec<u32> = Vec::new();
        let mut longest_path: Vec<u32> = Vec::new();
        let mut stack: Vec<u32> = Vec::new();
        let mut discovered: HashSet<u32> = HashSet::new();

        stack.push(search_index);
        loop {
            let current_vertex = stack.pop().unwrap();
            if !discovered.contains(&current_vertex) {

                path.push(current_vertex);

                discovered.insert(current_vertex);

                loop {
                    match self.neighbors.get(&current_vertex) {
                    Some(neighbors) => {
                        let new_neighbors: HashSet<_> = neighbors.difference(&discovered).collect();
                        println!("{:?}",new_neighbors );
                        if new_neighbors.len() == 0 {
                            println!("No neighbors found!");
                            path_candidates.push(path.clone());

                            if path.len() == 1 {
                                break;
                            }

                            let current_vertex = path.pop();
                        } else {
                            for neighbor in new_neighbors {
                                stack.push(neighbor.clone());
                            }
                            break;
                        }

                        
                    },
                    None => {}
                }
                }

               
              
            }


            if stack.len() == 0 {
                break;
            }
        }
        println!("{:?}", path_candidates);

        for path in path_candidates {
            if self.bridge_strength(&path) > self.bridge_strength(&longest_path) {
                    longest_path = path.clone();
            }
        }

        longest_path
    }

}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: String = args.get(1).unwrap().to_string();

    let f = File::open(filename).expect("file not found");
    let file = BufReader::new(&f);

    let components: Vec<(u32, u32)> = file.lines().map(|string| {
        let parts = string.unwrap().split("/").map(|value| value.parse().unwrap()).collect::<Vec<u32>>();
        (parts[0], parts[1])
    }).collect();

    println!("{:?}", components);

    // The plan.
    // I think one clever way of doing this is to construct a graph using the
    // components as edges in the graph. I can than traverse the graph determining
    // the strongest bridge. I think that this is a hard combinatorial problem,
    // but maybe I'll luck out.
    let mut graph: Graph = Graph::new();
    for component in components {
        graph.add_vertex(component);
    }

    // Construct the edges. In this case, we will place an edge between any two
    // vertices that have a corresponding number.
    let vertices = graph.vertices.clone();
    for (index1, vertex1) in &vertices {
        for (index2, vertex2) in &vertices {
            if index1 == index2 {
                continue;
            }
            if vertex1.0 == vertex2.0 ||  vertex1.0 == vertex2.1 ||  vertex1.1 == vertex2.0  || vertex1.1 == vertex2.1 {
                graph.add_edge((*index1, *index2));
            }
        } 
    }

    println!("{:?}", graph);

    // For each arbitrary starting point, perform a breadth-first search to find
    // the strongest bridge.
    let graph_vertices = graph.vertices.clone();
    let mut maximum_strength: u32 = 0;
    let mut strongest_bridge: Vec<u32> = Vec::new();
    for (vertex_id, vertex) in graph_vertices {
        let bridge_candidate = graph.strongest_bridge(vertex_id);
        let strength = graph.bridge_strength(&bridge_candidate);

        println!("{:?} | {:?} => {:?}", vertex_id, bridge_candidate, strength);

        if strength > maximum_strength {
            maximum_strength = strength;
            strongest_bridge = bridge_candidate.clone();
        }
        
    }
    println!("The strongest bridge has a strength of {}.", maximum_strength);
    println!("{:?}", strongest_bridge);
}
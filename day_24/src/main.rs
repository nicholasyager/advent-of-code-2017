use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

use std::collections::HashSet;
use std::collections::HashMap;

#[derive(Debug)]
struct Graph {
    edges: Vec<(u32, u32)>,
    vertices: HashSet<u32>,
    // I'm going to need to know the neighbors a lot, so we might as well cache 
    // a map of the neighbors for each vertex.
    neighbors: HashMap<u32, HashSet<u32>>
}

fn bridge_strength(bridge: &Vec<u32>) -> u32{
    let mut index = 0;
    let mut strength = 0;

    for element in bridge {
        strength += element;
    }
    strength
}

impl Graph {

    fn new() -> Graph {
        Graph {
            edges: Vec::new(),
            vertices: HashSet::new(),
            neighbors: HashMap::new()
        }
    }

    fn add_edge(&mut self, edge: (u32, u32)) {
        self.edges.push(edge);
        self.vertices.insert(edge.0);
        self.vertices.insert(edge.1);

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

    fn strongest_bridge(&self, search_index: u32) -> Vec<u32> {

        // 1  procedure DFS-iterative(G,v):
        // 2      let S be a stack
        // 3      S.push(v)
        // 4      while S is not empty
        // 5          v = S.pop()
        // 6          if v is not labeled as discovered:
        // 7              label v as discovered
        // 8              for all edges from v to w in G.adjacentEdges(v) do 
        // 9                  S.push(w)
        let mut path: Vec<u32> = Vec::new();
        let mut longest_path: Vec<u32> = Vec::new();
        let mut stack: Vec<u32> = Vec::new();
        let mut discovered: HashSet<u32> = HashSet::new();

        stack.push(search_index);
        loop {
            let current_vertex = stack.pop().unwrap();

            path.push(current_vertex);

            if !discovered.contains(&current_vertex) {

                if bridge_strength(&path) > bridge_strength(&longest_path) {
                    longest_path = path.clone();
                }
            

                discovered.insert(current_vertex);
                for neighbor in self.neighbors.get(&current_vertex).unwrap() {
                    stack.push(neighbor.clone());
                }
            }


            if stack.len() == 0 {
                break;
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
        graph.add_edge(component);
    }
    println!("{:?}", graph);

    // For each arbitrary starting point, perform a breadth-first search to find
    // the strongest bridge.
    let graph_vertices = graph.vertices.clone();
    let mut maximum_strength: u32 = 0;
    let mut strongest_bridge: Vec<u32> = Vec::new();
    for vertex in graph_vertices {
        let bridge_candidate = graph.strongest_bridge(vertex);
        let strength = bridge_strength(&bridge_candidate);
        if strength > maximum_strength {
            maximum_strength = strength;
            strongest_bridge = bridge_candidate.clone();
        }
        
    }
    println!("The strongest bridge has a strength of {}.", maximum_strength);
    println!("{:?}", strongest_bridge);
}
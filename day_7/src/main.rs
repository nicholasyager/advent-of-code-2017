extern crate regex;

use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Write;
use std::io::BufWriter;

use std::collections::HashSet;
use std::collections::HashMap;

use regex::Regex;

#[derive(Clone, Debug)]
struct Branch {
    id: String,
    weight: u32,
    total_weight: u32,
    leaves: HashSet<String>
}

impl Branch {
    fn new(id: String, weight: u32, leaves: HashSet<String>) -> Branch {
        let total_weight = 0;
        Branch{id, weight, total_weight, leaves}
    }
}

#[derive(Clone)]
struct Tree {
    branches: HashMap<String, Branch>
}

impl Tree {
    fn new() -> Tree {
        Tree{branches: HashMap::new()}
    }

    fn get_root(&self) -> Option<Branch> {

        let mut root: Option<Branch> = None;

        // For each branch
        for (branch_id, branch) in &self.branches {
            let mut is_root = true;

            // Check if any other branches have this branch as a leaf
            for (other_id, other_branch) in &self.branches {

                if other_id == branch_id {
                    continue;
                }
                if other_branch.leaves.contains(branch_id) {
                    is_root = false;
                    break
                }
                
            }

            if is_root {
                root = Some(branch.clone());
                break;
            }
        }

        root
    }

    fn find_unbalanced_branch(&self, id: String, root:bool, expected_weight: u32) {
        let branch = self.branches.get(&id).unwrap();


        // if (branch.total_weight - branch.weight) % branch.leaves.len() as u32 == 0_u32 {
        //     println!("Skipping all of {:?}, {:?} - {:?} / {:?}", branch.id, branch.total_weight, branch.weight, branch.leaves.len());
        //     return
        // }

       
        let mut weights: HashMap<u32, u32> = HashMap::new();
        for leaf_id in &branch.leaves {
            let leaf = self.branches.get(leaf_id).unwrap();
            let mut count = 1;
            if weights.contains_key(&leaf.total_weight) {
                count += weights.get(&leaf.total_weight).unwrap();
            }
            weights.insert(leaf.total_weight, count);
        }

        let mut leaf_weight: u32 = 0;
        let mut max_count: u32 = 0;
        for (weight, count) in &weights {
            if *count > max_count {
                leaf_weight = *weight;
                max_count = *count;
            }
        }

         // Check which leaf has the wrong weight. If they are all the same, THIS
        // branch has the wrong weight.

        let mut unbalanced_decendent = false;
        for leaf_id in &branch.leaves {
            let leaf = self.branches.get(leaf_id).unwrap();

            if leaf_weight != leaf.total_weight {
                // The unbalanced leaf is a decedent of this leaf. We must go
                // deeper.
                unbalanced_decendent = true;
                println!("Unbalanced decedent found. {:?}, {:?} != {:?}", leaf.id, leaf.total_weight, leaf_weight);
                self.find_unbalanced_branch(leaf.id.clone(), false, leaf_weight);
                break;
            } 

        }

        if !unbalanced_decendent && !root {
            let expected_branch_weight = (expected_weight as i32- branch.total_weight as i32) + branch.weight as i32;
            println!("The unbalanced branch is {:?}. It weighs {:?} but should weigh {:?}", branch.id, branch.weight, expected_branch_weight);
        }

    }

    fn write_dot(&self, filename: String) {
        let f = File::create(filename).expect("Error creating file.");
        let mut writer = BufWriter::new(&f);

        writeln!(&mut writer, "digraph G {{");
        for (branch_id, branch) in &self.branches {
            for leaf_id in &branch.leaves {
                writeln!(&mut writer, "\t {:?} -> {:?};", branch_id, leaf_id);
            }
        }
        for (branch_id, branch) in &self.branches {
            writeln!(&mut writer, "\t {:?} [label=\"{:?}\"];", branch_id, branch.total_weight);
        }
        writeln!(&mut writer, "}}");
    }

    fn compute_weights(&mut self, id: String) {
        let mut branch = self.branches.get_mut(&id).unwrap().clone();
        branch.total_weight += branch.weight;
        for leaf_id in &branch.leaves {
            let mut compute_leaf_weights = false;
            {
                let leaf = self.branches.get_mut(leaf_id).unwrap();
                if leaf.total_weight == 0 {
                    compute_leaf_weights = true;
                }
            }

            if compute_leaf_weights {
                self.compute_weights(leaf_id.to_string());
            }
            let leaf = self.branches.get_mut(leaf_id).unwrap();
            branch.total_weight += leaf.total_weight;
        }
        //println!("{:?}", branch);
        self.branches.insert(id, branch);
        
    }
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

    let mut tree = Tree::new();

    let re = Regex::new(r"\(|\)").unwrap();
    let comma_replace = Regex::new(r",").unwrap();

    for wrapped_line in file.lines() {

        let line = wrapped_line.unwrap();
        let line_parts = line.split_whitespace().collect::<Vec<&str>>();

        let id = line_parts[0].to_string();
        let weight: u32 = re.replace_all(line_parts[1], "").to_string().parse().unwrap();

        let mut leaves: HashSet<String> = HashSet::new();
        if line_parts.len() > 3 {
            for index in 3..line_parts.len() {
                let leaf = comma_replace.replace_all(line_parts[index], "").to_string();
                leaves.insert(leaf);
            }
        }

        tree.branches.insert(id.clone(), Branch{id, weight, leaves, total_weight: 0});

    }



    let root = tree.get_root();
    match root.clone() {
        Some(branch) => println!("The root is {:?}.", branch.id),
        None => println!("No root found!!")
    }

    let tree_root = root.unwrap();

    // Compute the tree's weights
    tree.compute_weights(tree_root.id.clone());

    let root = tree.get_root();
    let tree_root = root.unwrap();

    //let intital_expected = (tree_root.total_weight - tree_root.weight) / (tree_root.leaves.len() as u32);

    tree.find_unbalanced_branch(tree_root.id.clone(), true, tree_root.total_weight);

    //tree.write_dot("graph.dot".to_string());

    
    
}

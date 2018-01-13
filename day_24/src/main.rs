use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

use std::collections::HashSet;
use std::collections::HashMap;


fn generate_bridges(bridge: Vec<(u32, u32)>, components: Vec<(u32, u32)>) -> Vec<Vec<(u32, u32)>> {

    let mut bridges: Vec<Vec<(u32, u32)>> = Vec::new();

    for (index, component) in components.iter().enumerate() {
        let starting_piece = bridge[bridge.len()-1];

        if starting_piece.1 == component.0 {
            let mut new_components = components.clone();
            let _ = new_components.remove(index);
            let mut new_bridge = bridge.clone();
            new_bridge.push(*component);

            let new_bridges = generate_bridges(new_bridge, new_components);
            bridges.extend(new_bridges);
        } else if starting_piece.1 == component.1 {
            let mut new_components = components.clone();
            let _ = new_components.remove(index);
            let mut new_bridge = bridge.clone();
            new_bridge.push((component.1, component.0));

            let new_bridges = generate_bridges(new_bridge, new_components);
            bridges.extend(new_bridges);
        }
    }
    bridges.push(bridge);
    bridges
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

    // The plan:
    // The plan here is rather naive, but we might as well try. I'm going to
    // generate all possible combinations of bridges and select the strongest.
    // This will be rather slow, but so be it.

    let mut bridges: Vec<Vec<(u32, u32)>> = Vec::new();
    
    for (index, component) in components.iter().enumerate() {

        for version in 0..2 {
           
            let bridge_component: (u32, u32);
            if version == 0{
                bridge_component = *component;
                 
            } else {
                bridge_component = (component.1, component.0);
            }

            if bridge_component.0 != 0 {
                continue;
            }

            let mut component_pool = components.clone();
            let _ = component_pool.remove(index);
            let mut bridge: Vec<(u32, u32)> = Vec::new();

            bridge.push(bridge_component);
           
            println!("Exploring bridge {:?} ({:?})", bridge, component_pool);

            let new_bridges = generate_bridges(bridge, component_pool);
            bridges.extend(new_bridges);
        }

    }

    let mut maximum_length: u32 = 0;
    let mut maximum_length_strength: u32 = 0;

    let mut maximum_strength: u32 = 0;


    for bridge in &bridges {
        let mut strength: u32 = 0;
        for element in bridge {
            strength += element.0 + element.1;
        }

        if strength > maximum_strength {
            maximum_strength = strength;
        }

        if bridge.len() as u32 > maximum_length {
            maximum_length = bridge.len() as u32;
        } else if bridge.len() as u32 == maximum_length {
            if strength > maximum_length_strength {
                maximum_length = bridge.len() as u32;
                maximum_length_strength = strength;
            }
        }
    }

    println!("The strongest bridge has a strength of {}.", maximum_strength);
    println!("The longest bridge ({}) has a strength of {}", maximum_length, maximum_length_strength);


}
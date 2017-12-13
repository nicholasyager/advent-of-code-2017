use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::iter::Iterator;

#[derive(Clone, Debug)]
struct Layer {
    range: usize,
    scanner_position: usize,
    forward: bool
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


    let mut raw_layers: Vec<(u32, u32)> = Vec::new();

    for wrapped_line in file.lines() {
        let line = wrapped_line.unwrap();
        let parts = line.split(": ").collect::<Vec<&str>>();
        raw_layers.push((parts[0].parse().unwrap(), parts[1].parse().unwrap()));
    }

    let mut max_layers: u32 = 0;

    for raw_layer in &raw_layers {
        if raw_layer.0 > max_layers {
            max_layers = raw_layer.0;
        }
    }


    let mut firewall: Vec<Layer> = Vec::new();

    // println!("{:?}", raw_layers);
    // println!("{:?}", max_layers);

    // Instantiate as empty layers
    for index in 0..(max_layers+1) {
        firewall.push(Layer{range: 0, scanner_position: 0, forward: true});
    }

    // Populate the firewall with layers.
    for raw_layer in &raw_layers {
        let index = raw_layer.0 as usize;
        let layer = Layer{range: raw_layer.1 as usize, 
                          scanner_position: 0 as usize,
                          forward: true};
        // println!("Inserting layer {:?} into index {:?}.", layer, index);
        firewall[index] = layer;
    }

    let mut player_index: i32 = -1;
    let mut caught: u32 = 0;

    while player_index < max_layers as i32 {
        // Move player
        player_index += 1;
        println!("Player is now at layer {:?}.", player_index);
        {
            let current_layer = &firewall[player_index as usize];
            if current_layer.range != 0 {
                // Attempt to predict the scanner's position.
                let positive_cycle = (player_index / current_layer.range as i32) % 2 == 0;
                let predicted_position: i32;
                if positive_cycle {
                    predicted_position = player_index % current_layer.range as i32;
                } else {
                    predicted_position = current_layer.range as i32 - (player_index % current_layer.range as i32);
                }

                println!("Predicted position: {:?}", predicted_position);
            }


            println!("The scanner is at position {:?}.", current_layer.scanner_position);
            if current_layer.range != 0 && current_layer.scanner_position == 0 {
                println!("Player has been caught!");
                caught += 1;
            }
        }

        // Move scanners
        let mut new_firewall = firewall.clone();

        for (index, layer) in firewall.iter().enumerate() {
            let mut new_layer = layer.clone();

            // Empty ranges can be skipped
            if layer.range == 0 {
                continue
            }

            if layer.scanner_position == layer.range - 1 {
                new_layer.forward = false;
            } else if layer.scanner_position == 0 {
                new_layer.forward = true;
            }

            if new_layer.forward {
                new_layer.scanner_position += 1;
            } else {
                new_layer.scanner_position -= 1;
            }

            new_firewall[index] = new_layer;
        }
        firewall = new_firewall;
    }

    println!("Player was caught {:?} times.", caught);
}

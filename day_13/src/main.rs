use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::iter::Iterator;

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

    let mut firewall: Vec<u32> = Vec::new();

    // Instantiate as empty layers
    for _ in 0..(max_layers+1) {
        firewall.push(0);
    }

    // Populate the firewall with layers.
    for raw_layer in &raw_layers {
        let index = raw_layer.0 as usize;
        let layer = raw_layer.1;
        firewall[index] = layer;
    }

    let mut delay = 0;

    loop {
        let mut player_index: i32 = -1;
        let mut caught: u32 = 0;
        let mut time: u32 = delay;

        while player_index < max_layers as i32 {
            player_index += 1;
            let range = firewall[player_index as usize];

            if range != 0_u32 {
                let scanner_position: u32;
                let offset = time % ((range  - 1) * 2);

                if offset > range {
                    scanner_position = 2 * (range - 1) - offset;
                } else {
                    scanner_position = offset;
                }

                if scanner_position == 0 {
                    if player_index == 0 {
                        caught += 1;
                    }
                    caught += range * player_index as u32;
                }
            }
           
            time += 1
        }
       
        if delay == 0 {
            println!("Delay: {:?} - Player was caught with a severity of {:?}.", delay, caught);
        }

        if caught == 0 {
            break;
        } 
        delay += 1;
    }
    println!("Delay: {:?} - Player evaded capture.", delay);
}

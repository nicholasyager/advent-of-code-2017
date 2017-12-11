use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: String = match args.get(1)  {
        None => panic!("You must supply a file to evaluate."),
        Some(filename) => filename.clone()
    };

    println!("Processing file {:?}", filename);

    let f = File::open(filename).expect("file not found");
    let file = BufReader::new(&f);

    let mut registers: HashMap<String, i32> = HashMap::new();
    let mut register_maximum: i32 = 0;

    for wrapped_line in file.lines() {
        let line = wrapped_line.unwrap();
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        
        // Set all of our values
        let register_1 = parts[0].to_string();
        let instruction = parts[1].to_string();
        let value_1: i32 = parts[2].to_string().parse().unwrap();

        let register_2 = parts[4].to_string();
        let conditional = parts[5].to_string();
        let value_2: i32 = parts[6].to_string().parse().unwrap();

        // Let's start processing.

        // Get the value of register 1
        let register_1_value: i32 = match registers.get(&register_1) {
            Some(value) => value.clone(),
            None => 0_i32
        };

        // Get the value of register 2
        let register_2_value: i32 = match registers.get(&register_2) {
            Some(value) => value.clone(),
            None => 0_i32
        };

        // Perform the conditional
        let condition = match conditional.as_ref() {
            ">" => register_2_value > value_2,
            "<" => register_2_value < value_2,
            "==" => register_2_value == value_2,
            "<=" => register_2_value <= value_2,
            ">=" => register_2_value >= value_2,
            "!=" => register_2_value != value_2,
            _ => panic!("Unknown conditional detected! {:?}", conditional)
        };

        if !condition {
            continue;
        }

        // Perform the instruction
        let new_value = match instruction.as_ref() {
            "inc" => register_1_value + value_1,
            "dec" => register_1_value - value_1,
            _ => panic!("Unknown instruction detected! {:?}", instruction)
        };

        if new_value > register_maximum {
            register_maximum = new_value;
        }
        registers.insert(register_1, new_value);

    }

    let mut max_value: i32 = 0;
    for (register, value) in &registers {
        if *value > max_value {
            max_value = *value;
        }
    }

    println!("The largest register value is {:?}.", max_value);
    println!("The historic register maximum was {:?}", register_maximum);
}

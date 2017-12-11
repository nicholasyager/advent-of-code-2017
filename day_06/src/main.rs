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
    let mut file = BufReader::new(&f);

    let mut registers: Vec<u32> = Vec::new();

    let mut line = String::new();
    let _ = file.read_line(&mut line);
    for count in line.split_whitespace().collect::<Vec<&str>>() {
        registers.push(count.parse().unwrap());
    }

    let mut states: HashMap<Vec<u32>, u32> = HashMap::new();
    let mut cycles: u32 = 0;
    let cycle_size: u32;

    // Store the original registers vector
    states.insert(registers.clone(), cycles);

    loop {
        // Find the largest register. If there is a tie, go with the smallest.
        let mut largest_register: usize = 0;
        let mut largest_register_size: u32 = 0;

        for (index, size) in registers.iter().enumerate() {
            if *size > largest_register_size {
                largest_register = index;
                largest_register_size = *size;
            }
        }

        // Redistribute the values to the right. Loop over boundaries in the
        // vector.

        registers[largest_register] = 0;

        while largest_register_size > 0 {
            largest_register += 1;

            if largest_register == registers.len() {
                largest_register = 0;
            }

            registers[largest_register] += 1;
            largest_register_size -= 1;
        }

        cycles += 1;

        // Check if the states HashSet contains the registers vector already. If
        // it does, break out. If not, continue.
        if states.contains_key(&registers) {
            cycle_size = cycles - states.get(&registers).unwrap();
            break
        } else {
            states.insert(registers.clone(), cycles);
        }

        
    }
    println!("Loop detected in {:?} cycles. Cycle size is {:?}.", cycles, cycle_size);
    println!("{:?}", registers)

}

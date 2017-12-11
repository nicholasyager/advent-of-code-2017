use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: String = match args.get(1)  {
        None => panic!("You must supply a file to evaluate."),
        Some(filename) => filename.clone()
    };

    println!("Processing file {:?}", filename);

    let f = File::open(filename).expect("file not found");
    let file = BufReader::new(&f);

    let mut instructions: Vec<i32> = Vec::new();

    for line in file.lines() {
        let instruction: i32 = line.unwrap().parse().unwrap();
        instructions.push(instruction);
    }
    //println!("{:?}", instructions);

    // Evaluate the instructions;
    let mut cursor: u32 = 0;
    let mut steps: u32 = 0;

    while cursor < instructions.len() as u32 {
        //println!("Current instruction: {:?} ({:?})",instructions[cursor as usize],cursor);
        let new_cursor =  (cursor as i32 + instructions[cursor as usize]) as u32;

        let increment: i32;
        if instructions[cursor as usize] >= 3 {
            increment = -1;
        } else {
            increment = 1;
        }

        instructions[cursor as usize] += increment;
        cursor = new_cursor;
        steps += 1;
    }
    println!("Escaped in {:?} steps.", steps);
    println!("{:?}", instructions);
}

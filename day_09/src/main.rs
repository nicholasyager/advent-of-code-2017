use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
extern crate colored;

use colored::*;

#[derive(Debug, Clone)]
enum States {
    GROUPING,
    GARBAGE,
    SKIPING
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

    let mut stream: Vec<char> = Vec::new();

    for line in file.lines() {
        for c in line.expect("lines failed").chars() {
            stream.push(c);
        }
    }

    let mut state: Vec<States> = Vec::new();

    // The default state is GROUPING.
    state.push(States::GROUPING);
    let mut group_value: u32 = 0;
    let mut index: usize = 0;
    let mut score: u32 = 0;

    let mut garbage: u32 = 0;

    while index < stream.len() {

        let current_states = state.clone();
        let current_state = current_states.last().unwrap();
        let character: char = stream[index];

        match *state.last().unwrap() {
            States::SKIPING  => print!("{}", character.to_string().red()),
            States::GARBAGE  => print!("{}", character.to_string().yellow()),
            States::GROUPING => print!("{}", character.to_string().green()),
        }

        match *current_state {
            States::SKIPING => {
                let _ = state.pop();
            },
            States::GROUPING => {
                if character == '{' {
                    state.push(States::GROUPING);
                    group_value += 1;
                    score += group_value;
                } else if character == '}' {
                    group_value -= 1;
                    let _ = state.pop();
                } else if character == '<' {
                    state.push(States::GARBAGE);
                } else if character == '!' {
                    state.push(States::SKIPING);
                } 
            }, States::GARBAGE => {
                if character == '>' {
                    let _ = state.pop();
                } else if character == '!' {
                    state.push(States::SKIPING);
                } else {
                    garbage += 1;
                }
            }
        }
        index += 1;
    }

    println!("\nScore: {:?}", score);
    println!("Garbage: {:?}", garbage);

}

use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::iter::Iterator;

use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Virus {
	position: (i32, i32),
	direction: usize,
	infections: u32
}

impl Virus {

	fn walk(&mut self) {
		// Move forward.
		let directions: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
		self.position = ((self.position.0 + directions[self.direction].0),
						 (self.position.1 + directions[self.direction].1));
	}

	fn work(&mut self, current_state: u8) -> u8 {
		/// Infect or clean
		let future_state: u8;

		if current_state == 3 {
			future_state = 0;
		} else {
			future_state = current_state + 1;
		}

		if future_state == 2 {
			self.infections += 1;
		}
		future_state
	}

	fn update_direction(&mut self, current_state: u8) {
		// Determine a direction.
		let direction: usize;
		if current_state  == 2 {
			// The current cell is infected, turn right.
			if self.direction == 3 {
				self.direction = 0;
			} else {
				self.direction += 1;
			}
		} else if current_state == 0 {
			// The current cell is infected, turn left.
			if self.direction == 0 {
				self.direction = 3;
			} else {
				self.direction -= 1;
			}
		} else if current_state == 3 {
			self.direction = match self.direction {
				0 => 2,
				1 => 3,
				2 => 0,
				3 => 1,
				_ => panic!("Unknown direction provided.")
			};
		}
	}
}

fn print_map(map: &HashMap<(i32, i32), u8>) {
	let symbols: [char; 4] = ['.', 'W', 'I', 'F'];
	for y in (-25_i32..25_i32).rev() {
		for x in -25_i32..25_i32  {
			let state: u8 = match map.get(&(x, y)) {
				Some(state) => *state,
				None => 0
			};
			print!("{}", symbols[state as usize]);
		}
		println!("");
	}
}

fn main() {

	let args: Vec<String> = env::args().collect();
    let filename: String = args.get(1).unwrap().clone();
    let iterations: u32 = args.get(2).unwrap().parse().unwrap();
    
    // Load the map
    let f = File::open(filename.clone()).expect("file not found");
    let file = BufReader::new(&f);

    let mut map_vector: Vec<u8> = Vec::new();
    for wrapped_line in file.lines() {
       	for character in wrapped_line.unwrap().chars() {
       		let state: u8 = if character == '#' {2} else {0};
       		map_vector.push(state);
       	}
    }

    let width = (map_vector.len() as f64).sqrt() as u32;
    let midpoint: i32 = (width  as i32 - 1) / 2;
    let mut map: HashMap<(i32, i32), u8> = HashMap::new();

    let mut x: i32 = -midpoint;
    let mut y: i32 = midpoint;

     for status in map_vector {
    	let position = (x, y);
    	println!("{:?} {}", position, status);
    	map.insert(position, status);

    	x += 1;

    	if x > midpoint {
    		x = -midpoint;
    		y -= 1;
    	}
    }

    println!("Loaded the map {:?} ({}x{})", filename, width, width);
    print_map(&map);
   
    // Create the virus

	let mut virus: Virus = Virus{position: (0_i32, 0_i32),
								 direction: 0_usize,
								 infections: 0_u32};
	// Simulate the virus progression.

	for _ in 0..iterations {

		let state: u8 = match map.get(&virus.position) {
			Some(state) => *state,
			None => 0
		};

		virus.update_direction(state);

		let new_state = virus.work(state);
		map.insert(virus.position, new_state);
		
		virus.walk();
	}
	print_map(&map);
	println!("{:?}", virus);

}

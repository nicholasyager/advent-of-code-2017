use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Processor {
	registers: HashMap<String, i64>,
	index: usize,
	skip: i32,
	multiplications: u32
}

impl Processor {

	fn new() -> Processor {
		Processor{
			index: 0_usize, 
			registers: HashMap::new(), 
			multiplications: 0_u32,
			skip: 1_i32
		}
	}

	fn execute(&mut self, command: String, parameter1: String, parameter2: String) {
		//println!("{:?} {:?} {:?} {:?}", self.index, command, parameter1, parameter2);

		let processed_parameter = self.clone().process_parameter(parameter2);
		self.skip = 1_i32;

		match command.as_ref() {
			"set" => self.set(parameter1, processed_parameter),
			"mul" => self.mul(parameter1, processed_parameter),
			"sub" => self.sub(parameter1, processed_parameter),
			"jnz" => self.jnx(parameter1, processed_parameter),
			_ => panic!("Unable to process command {:?}.", command)
		}

		self.index = ( self.index as i32 + self.skip) as usize;
	}

	fn process_parameter(self, parameter: String) -> i64 {
		let value: i64 = match parameter.parse() {
            Ok(value) => value,
            Err(_) => {
                // Lookup the value.
                match self.registers.get(&parameter.to_string()) {
                    Some(x) => *x,
                    None => 0_i64
                }
            }
        };
        value
	}

	fn set(&mut self, register: String, value: i64) {
		self.registers.insert(register, value);
	}

	fn mul(&mut self, register: String, value: i64) {
		let register_value = self.registers.get(&register).unwrap_or(&0_i64).clone();
		self.set(register, register_value*value);
		self.multiplications += 1;
	}

	fn sub(&mut self, register: String, value: i64) {
		let register_value = self.registers.get(&register).unwrap_or(&0_i64).clone();
		self.set(register, register_value-value);
	}

	fn jnx(&mut self, parameter1: String, value: i64) {
		let trigger = self.clone().process_parameter(parameter1);
		if trigger != 0 {
			self.skip = value as i32;
		}
	}

}

fn main() {

	let args: Vec<String> = env::args().collect();
    let filename: String = args.get(1).unwrap().to_string();

    let f = File::open(filename).expect("file not found");
    let file = BufReader::new(&f);

    let commands: Vec<(String, String, String)> = file.lines().map(|string| {
    	match string {
    		Ok(string_part) => {
    			let parts: Vec<&str> = string_part.split_whitespace().collect();
		    	if parts.len() != 3 {
		    		panic!("Invalid command detected!");
		    	}

		    	(parts[0].to_string(), parts[1].to_string(), parts[2].to_string())
    		},
    		Err(_) => panic!("Picnic!")
    	}
    }).collect::<Vec<(String, String, String)>>();

    let mut processor = Processor::new();
    //processor.set(String::from("a"), 1);

    loop {
    	let command = match commands.get(processor.index) {
    		Some(command) => command,
    		None => break
    	};
    	processor.execute(command.0.clone(), command.1.clone(), command.2.clone());
    }

    println!("The processor has multiplied {} times.", processor.multiplications);
    println!("The value of 'h' is {}.", processor.registers.get("h").unwrap());
}

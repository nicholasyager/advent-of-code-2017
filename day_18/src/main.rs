use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Program {
    active: bool,
    index: usize,
    output: Option<i64>,
    input_queue: Vec<i64>,
    registers: HashMap<String, i64>,
    recovered_sounds: u32,
    sent_sounds: u32
}

impl Program {
    fn new() -> Program {
        Program{
            active: true,
            index: 0_usize,
            output: None,
            input_queue: Vec::new(),
            registers: HashMap::new(),
            recovered_sounds: 0_u32,
            sent_sounds: 0_u32
        }
    }

    fn parse_command(&self, command: String) -> (String, String, i64) {
        let parts = command.split_whitespace().collect::<Vec<&str>>();

        let command = parts[0].to_string();
        let parameter1: String = parts[1].to_string();
        // We don't always know what parameter 2 will be. In the case the parsing
        // fails, we treat it like a char and lookup what the value should be.
        let parameter2: i64 = match parts.get(2) {
            Some(index) => {
                
                match index.parse() {
                    Ok(value) => value,
                    Err(error) => {
                        // Lookup the value.
                        match self.registers.get(&index.to_string()) {
                            Some(x) => *x,
                            None => 0_i64
                        }
                    }
                }
            },
            None => 0_i64
        };
        (command, parameter1, parameter2)
    }

    fn process_command(&mut self, raw_command: String) {
        
        //let mut new_program = self.clone();
        let mut skip = 1;
        self.output = None;

        let parsed_command = self.parse_command(raw_command.clone());
        let command = parsed_command.0;
        let parameter1 = parsed_command.1;
        let parameter2 = parsed_command.2;

        //println!("{} - {} {} {}", self.index, command, parameter1, parameter2);

        // Compute the instruction
        let new_value: Option<i64> = match command.as_ref() {
            "snd" => {
                let parts = raw_command.split_whitespace().collect::<Vec<&str>>();
                let current_value: i64 = match parts.get(1) {
                    Some(index) => {
                        
                        match index.parse() {
                            Ok(value) => value,
                            Err(error) => {
                                // Lookup the value.
                                match self.registers.get(&index.to_string()) {
                                    Some(x) => *x,
                                    None => 0_i64
                                }
                            }
                        }
                    },
                    None => 0_i64
                };

                // Set the queue
                self.output = Some(current_value);

                self.sent_sounds += 1;

                // Return without doing anything.
                None
            },
            "set" => Some(parameter2),
            "add" => Some(self.registers.get(&parameter1.to_string()).unwrap_or(&0_i64).clone() + parameter2),
            "mul" => Some(self.registers.get(&parameter1.to_string()).unwrap_or(&0_i64).clone() * parameter2),
            "mod" => Some(self.registers.get(&parameter1.to_string()).unwrap_or(&0_i64).clone() % parameter2),
            "rcv" => {
                // Load the next value in the input queue. If it is not 0, 
                // set it in the appropriate register. Return otherwise.
                if self.input_queue.len() > 0 {
                    let receive_value: i64 = self.input_queue.pop().unwrap();

                    if self.recovered_sounds == 0 {
                        println!("The first recovered sound is {:?}", receive_value);
                        self.recovered_sounds += 1;
                    }
                    Some(receive_value)

                } else {
                    self.active = false;
                    None
                }
                
            },
            "jgz" => {
                let current_value: i64 = self.registers.get(&parameter1.to_string()).unwrap_or(&0_i64).clone();
                if current_value > 0 {
                    skip = parameter2;
                }
                Some(current_value)
            },
            _ => panic!("Unknown instruction found!")
        };

        match new_value {
            Some(value) => {self.registers.insert(parameter1, value);},
            None => (),
        }
        

        self.index =  (self.index as i64 + skip) as usize;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: String = args.get(1).unwrap().to_string();

    let f = File::open(filename).expect("file not found");
    let file = BufReader::new(&f);

    let mut program1 = Program::new();
    program1.registers.insert(String::from("p"), 0);

    let mut program2 = Program::new();
    program2.registers.insert(String::from("p"), 1);

    let mut registers: HashMap<String, i64> = HashMap::new();
    

    let mut recovered_sounds: u64 = 0;
    let mut commands: Vec<String> = Vec::new();

    for wrapped_line in file.lines() {
        let command_string = wrapped_line.unwrap();
       
        commands.push(command_string.clone());
    }
    loop {
        let mut terminate = false;
        for program in 0..2 {
            let other_program: usize;
            
            if program == 0 {
                let command_string = commands.get(program1.index).expect("Index doesn't exist. Terminating");
                program1.process_command(command_string.clone());
                match program1.output {
                    Some(value) => {program2.input_queue.insert(0_usize, value);},
                    None => {}
                }

                if !program1.active {
                    terminate = true;
                    break;
                }

            } else {
                let command_string = commands.get(program2.index).expect("Index doesn't exist. Terminating");
                program2.process_command(command_string.clone());
                match program2.output {
                    Some(value) => {program1.input_queue.insert(0_usize, value);},
                    None => {}
                }
            }
        }

        println!("{:?}", program2.input_queue.len());

        if terminate {
            println!("terminating");
            break;
        }
    }
    println!("Program 1 has sent a value {} times.", program2.sent_sounds);
}

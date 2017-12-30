use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;

struct Program {
    send_registers: HashMap<String, i64>,
    receive_registers: HashMap<String, Vec<i64>>,
    data_registers: HashMap<String, i64>,
}

impl Program {
    fn new() -> Program {
        Program{
            send_registers: HashMap::new(),
            receive_registers: HashMap::new(),
            data_registers: HashMap::new(),
            index: 0_usize,

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

    fn process_command(&self, command: String) -> Program {
        
        let mut new_program = self.clone();

        let parsed_command = self.parse_command(command);
        let command = parsed_command.0;
        let parameter1 = parsed_command.1;
        let parameter2 = parsed_command.2;

        println!("{} - {} {} {}", self.index, command, parameter1, parameter2);

        // Compute the instruction
        let new_value: i64 = match command.as_ref() {
            "snd" => {
                let current_value: i64 = self.data_registers.get(&parameter1.to_string()).unwrap_or(&0_i64).clone();

                // Set the queue
                new_program.send_registers.insert(parameter1.clone(), current_value);

                // Return without doing anything.
                return
            },
            "set" => parameter2,
            "add" => self.data_registers.get(&parameter1.to_string()).unwrap_or(&0_i64).clone() + parameter2,
            "mul" => self.data_registers.get(&parameter1.to_string()).unwrap_or(&0_i64).clone() * parameter2,
            "mod" => self.data_registers.get(&parameter1.to_string()).unwrap_or(&0_i64).clone() % parameter2,
            "rcv" => {
                // Load the next value in the receive buffers. If it is not 0, 
                // set it in the appropriate register. Return otherwise.
                let register_of_interest = self.receive_registers.get(&parameter1.to_string());
                let (receive_value, remaining_values) = register_of_interest.split_first().unwrap();

                if receive_value > 0 {
                    if recovered_sounds == 0 {
                        println!("The first recovered sound is {:?}", receive_value);
                        recovered_sounds += 1;
                    }
                    receive_value
                } else {
                    return
                }
            },
            "jgz" => {
                let current_value: i64 = self.data_registers.get(&parameter1.to_string()).unwrap_or(&0_i64).clone();
                if current_value > 0 {
                    skip = parameter2;
                }
                current_value
            },
            _ => panic!("Unknown instruction found!")
        };
        self.data_registers.insert(parameter1, new_value);

        new_program.index =  (self.index as i64 + skip) as usize;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: String = args.get(1).unwrap().to_string();

    let f = File::open(filename).expect("file not found");
    let file = BufReader::new(&f);

    let mut program1 = Program::new();
    let mut program2 = Program::new();

    let mut registers: HashMap<String, i64> = HashMap::new();
    

    let mut recovered_sounds: u64 = 0;
    let mut commands: Vec<String> = Vec::new();

    for wrapped_line in file.lines() {
        let command_string = wrapped_line.unwrap();
       
        commands.push(command_string.clone());
    }

    let mut index = 0;
    loop {

        for program in 0..2 {

            let other_program: usize;

            let mut skip = 1;
            let command_string = commands.get(index).expect("Index doesn't exist. Terminating");
            if program == 0 {
                program1 = program1.process_command(command_string);

            } else {
                program2 = program1.process_command(command_string);
            }
        }
    }
}

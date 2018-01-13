use std::collections::HashMap;

#[derive(Debug)]
enum States {
	A,
	B,
	C,
	D,
	E,
	F
}

fn main() {

	let steps: u32 = 12861455;

    let mut tape: HashMap<i64, bool> = HashMap::new();

    let mut cursor: i64 = 0;
    let mut state: States = States::A;


    for step in 0..steps {
    	let current_tape = tape.clone();
    	let current_value = current_tape.get(&cursor).unwrap_or(&false);

    	//println!("{} - {:?} {:?} {:?}", step, state, cursor, current_value);

    	let rule: (bool, i8, States) = match state {
    		States::A => {
    			if !current_value {
    				(true, 1_i8, States::B)
    			} else {
    				(false, -1_i8, States::B)
    			}
    		},
    		States::B => {
    			if !current_value {
    				(true, -1_i8, States::C)
    			} else {
    				(false, 1_i8, States::E)
    			}
    		},
    		States::C => {
    			if !current_value {
    				(true, 1_i8, States::E)
    			} else {
    				(false, -1_i8, States::D)
    			}
    		},
    		States::D => {
    			(true, -1_i8, States::A)
    		},
    		States::E => {
    			if !current_value {
    				(false, 1_i8, States::A)
    			} else {
    				(false, 1_i8, States::F)
    			}
    		},
    		States::F => {
    			if !current_value {
    				(true, 1_i8, States::E)
    			} else {
    				(true, 1_i8, States::A)
    			}
    		}
    	};

    	tape.insert(cursor, rule.0);
    	cursor += rule.1 as i64;
    	state = rule.2;
    }

    let mut checksum = 0;
    for (key, value) in &tape {
    	if *value {
    		checksum += 1;
    	}
    }
    println!("The checksum is {}.", checksum);


}

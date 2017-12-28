use std::env;


fn main() {
 	let args: Vec<String> = env::args().collect();
    let steps: u32 = args.get(1).unwrap().parse().unwrap();
    let rounds: u32 = args.get(2).unwrap().parse().unwrap();
    let part: u32 = args.get(3).unwrap().parse().unwrap();

    let mut buffer: Vec<u32> = vec![0];

    println!("Input: {steps}", steps = steps);

    // The spinlock algorithm.
    // Step forward n steps before iunserting the next value (m+1). Continuing
    // from this position, step forward n more steps and insert the next value
    // (m+2). This will slowly grow the circular buffer by 1 for each round.
    // We want to run this 2017 times.

    let mut current_position = 0;
    let mut current_value = 1;
    let mut position_of_interest = 0;
    let mut round = 1;

    while current_value <= rounds {
	    current_position = 1 + (steps + current_position) % round;
	    
	    if part == 2 {
	    	if current_position == 1 {
	    		position_of_interest = current_value;
	    	}
	    } else {
	    	buffer.insert(current_position as usize , current_value);
	    }
	    current_value += 1;
	    round += 1;
	}

	if part == 2 {
		println!("{:?}", position_of_interest);
    } else {
    	println!("{:?}", buffer[current_position as usize+1]);
    }

	
}

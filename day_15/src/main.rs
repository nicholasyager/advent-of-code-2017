use std::env;
use std::iter::Iterator;

struct Generator {
	multiplication_factor: u64
}

impl Generator {
	fn generate(&self, previous_value: u64) -> u64 {
		(previous_value * self.multiplication_factor) % 2147483647_u64
	}
}

#[cfg(test)]
mod tests {
	use super::*;

    #[test]
    fn test_generation() {
    	let generator = Generator{multiplication_factor: 16807};
    	assert_eq!(1092455, generator.generate(65));
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut generator_a_value: u64 = args.get(1).unwrap().parse().unwrap();
    let mut generator_b_value: u64 = args.get(2).unwrap().parse().unwrap();
    let cycles: u64 = args.get(3).unwrap().parse().unwrap();

    println!("Generator A: {}, Generator B: {}.", generator_a_value, generator_b_value);

    let generator_a = Generator{multiplication_factor: 16807};
    let generator_b = Generator{multiplication_factor: 48271};

    let mut matches = 0;

    for cycle in 0..cycles {
    	generator_a_value = generator_a.generate(generator_a_value);
    	generator_b_value = generator_b.generate(generator_b_value);

    	if ((generator_a_value & 0xFFFF) == (generator_b_value & 0xFFFF)) {
    		matches +=1 ;
    	}

    }

    println!("{} matches were found!", matches);
}

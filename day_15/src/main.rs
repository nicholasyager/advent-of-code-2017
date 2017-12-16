use std::env;
use std::iter::Iterator;

struct Generator {
	multiplication_factor: u64,
	common_denominator: u64
}

impl Generator {
	fn generate(&self, previous_value: u64) -> u64 {
		let mut computing_value = previous_value;
		loop {
			let generated_value = (computing_value * self.multiplication_factor) % 2147483647_u64;
			if generated_value % self.common_denominator == 0 {
				return generated_value
			}
			computing_value = generated_value
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

    #[test]
    fn test_generation() {
    	let generator = Generator{multiplication_factor: 16807, common_denominator: 4};
    	assert_eq!(1352636452, generator.generate(65));
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut generator_a_value: u64 = args.get(1).unwrap().parse().unwrap();
    let mut generator_b_value: u64 = args.get(2).unwrap().parse().unwrap();
    let cycles: u64 = args.get(3).unwrap().parse().unwrap();

    println!("Generator A: {}, Generator B: {}.", generator_a_value, generator_b_value);

    let generator_a = Generator{multiplication_factor: 16807, common_denominator: 4};
    let generator_b = Generator{multiplication_factor: 48271, common_denominator: 8};

    let mut matches = 0;

    for _ in 0..cycles {
    	generator_a_value = generator_a.generate(generator_a_value);
    	generator_b_value = generator_b.generate(generator_b_value);

    	if (generator_a_value & 0xFFFF) == (generator_b_value & 0xFFFF) {
    		matches +=1 ;
    	}

    }

    println!("{} matches were found!", matches);
}

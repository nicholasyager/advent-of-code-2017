use std::env;

fn main() {

    let args: Vec<String> = env::args().collect();

    let sequence: Vec<u32> = match args.get(1)  {
        None => panic!("You must supply a sequence to evaluate."),
        Some(sequence_string) => {
            let sequence_char = sequence_string.split("").collect::<Vec<&str>>();
            let mut sequence: Vec<u32> = Vec::new();

            for character in sequence_char {
                if character == "" {
                    continue;
                }
                sequence.push(character.parse::<u32>().unwrap());
            }
            sequence
        }
    };

    let mut sum: u32 = 0;

    let offset = sequence.len() / 2;

    for (index, value) in sequence.iter().enumerate() {
        let mut next_index = index + offset;
        if next_index >= sequence.len() {
            next_index = next_index - sequence.len();
        }

        let next_value = sequence.get(next_index).unwrap();
        if value == next_value {
            sum += value;
        }

    }
    println!("{:?}", sum);
}

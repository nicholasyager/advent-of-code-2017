use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let hash_input: String = match args.get(1)  {
        None => panic!("You must supply a file to evaluate."),
        Some(filename) => filename.clone()
    };

    // Generate each of the hash inputs
    let hash_inputs: Vec<> = (0..127).map(|row| {
        format!("{:?}-{:?}", hash_input, row)
    }).collect();
    println!("{:?}", hash_inputs);

    // Generate a knot hash for each input

    // Convert each knot hash into binary

    // Take the sum of each row.
}

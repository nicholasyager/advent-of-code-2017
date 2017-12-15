use std::env;

fn to_binary(number: char) -> Vec<u8> {
    let mut binary_vector: Vec<u8> = Vec::new();
    let mut result = number.to_digit(16).unwrap();

    while result > 0 {
        if result % 2 == 0 {
            binary_vector.push(0);
        } else {
            binary_vector.push(1);
        }

        result = result / 2;
    }

    binary_vector.reverse();
    binary_vector
} 

fn hash(hash_input: String) -> String {

    println!("{:?}", hash_input);

    // Convert hash_input into a vector of lengths;
    let mut lengths: Vec<u8> = Vec::new();
    for byte in hash_input.as_bytes() {
        lengths.push(*byte);
    }
    println!("{:?}", lengths);

    let mut rope: Vec<u8> = Vec::new();

    let rope_size: usize = 256;

    for index in 0..rope_size {
        rope.push(index as u8);
    }

    let mut cursor: usize = 0;
    let mut skip: usize = 0;

    for _ in 0..64 {
        for length_byte in &lengths {
            let length = *length_byte;

            let mut sublist: Vec<u8> = Vec::new();

            // Get a vector of the elements between cursor and cursor + length.
            if cursor + length as usize >= rope_size {
                let end_1 = rope_size;
                let end_2 = length as usize - (rope_size-cursor);
                sublist.extend_from_slice(&rope.get(cursor..end_1).unwrap());
                sublist.extend_from_slice(&rope.get(0..end_2).unwrap());

            } else {
                sublist.extend_from_slice(rope.get(cursor..(cursor+length as usize)).unwrap());
            }

            // Reverse the slice
            sublist.reverse();

            // Insert the elements of the slice back into place.
            let mut insertion_cursor = cursor;
            for value in &sublist {

                if insertion_cursor >= rope.len() {
                    insertion_cursor -= rope.len();
                }

                rope[insertion_cursor] = *value;
                insertion_cursor += 1;
            }

            // Update the cursor position by the skip size.
            cursor += length as usize + skip;

            // Do a quick cursor boundary check.
            while cursor >= rope.len() {
                cursor -= rope.len();
            }

            // Increment the skip size.
            skip += 1;
        }
    }

    let mut hash: String = String::from("");

    let mut hash_vector: Vec<String> = Vec::new();
    for block in 0..(rope.len()/16) {
        let mut hash_byte = 0;
        for index in 0..16 {
            hash_byte ^= rope[(block*16)+index];
        }
        hash += &(format!("{:02x}", hash_byte));
    }

    hash
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let hash_input: String = match args.get(1)  {
        None => panic!("You must supply a file to evaluate."),
        Some(filename) => filename.clone()
    };

    // Generate each of the hash inputs
    let hash_inputs: Vec<String> = (0..128).map(|row| {
        format!("{}-{}", hash_input, row)
    }).collect();
    println!("{:?}", hash_inputs);

    // Generate a knot hash for each input
    let hashes: Vec<String> = hash_inputs.iter().map(|hash_input| {
        hash(hash_input.clone())
    }).collect();
    println!("{:?}", hashes);
    
    // Convert each knot hash into binary
    let bitmap: Vec<Vec<Vec<u8>>> = hashes.iter().map(|hash| {
       hash.chars().map(|value| to_binary(value)).collect()
    }).collect();
    println!("{:?}", bitmap);

    // Take the sum of each row.
    let mut map_sum: u32 = 0;
    for row in bitmap {
        for column in row {
            for digit in column {
                map_sum += digit as u32;
            }
        }
    }

    println!("The number of \"on\" cells is {:?}.", map_sum);
}

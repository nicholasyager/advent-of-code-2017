use std::env;
use std::fs::File;
use std::io::Read;
use std::iter::Iterator;

fn hash_vector(vector: Vec<u8>) -> Vec<u8> {
    let mut hash_byte_vector: Vec<u8> = Vec::new();
    for block in 0..(vector.len()/16) {
        let mut hash_byte = 0;
        for index in 0..16 {
            hash_byte ^= vector[(block*16)+index];
        }
        hash_byte_vector.push(hash_byte);
    }
    hash_byte_vector
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_function() {
        let test_vector: Vec<u8> = vec![65, 27, 9, 1, 4, 3, 40, 50, 91, 7, 6, 0, 2, 5, 68, 22];
        assert_eq!(64_u8, hash_vector(test_vector)[0]);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: String = match args.get(1)  {
        None => panic!("You must supply a file to evaluate."),
        Some(filename) => filename.clone()
    };

    println!("Processing file {:?}", filename);

    let mut file = File::open(filename).expect("file not found");
    let mut lengths: Vec<u8> = Vec::new();
    
    let len = file.read_to_end(&mut lengths);

    let nonce: [u8; 5] = [17, 31, 73, 47, 23];
    lengths.extend_from_slice(&nonce);
    println!("Found {:?} bytes:", len.unwrap());


    for (index, byte) in lengths.iter().enumerate() {
        if index % 16 == 0{
            print!("\n");
        }
        print!("{:02x} ", byte);
    }
    println!("\n");

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
    let hash_byte_vector = hash_vector(rope);
    
    for hash_byte in &hash_byte_vector {
        print!("{:02x}", hash_byte);
    }
}

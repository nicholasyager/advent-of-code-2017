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
    while binary_vector.len() < 4 {
        binary_vector.insert(0, 0);
    }
    binary_vector
} 



fn hash(hash_input: String) -> String {
    let rope_size: usize = 256;
    let mut rope: Vec<u8> = (0..rope_size).map(|x| x as u8).collect();

    let mut cursor: usize = 0;
    let mut skip: usize = 0;

    let nonce: [u8; 5] = [17, 31, 73, 47, 23];
    let mut lengths: Vec<u8> = Vec::from(hash_input.as_bytes());
    lengths.extend_from_slice(&nonce);

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
    for block in 0..(rope.len()/16) {
        let mut hash_byte = 0;
        for index in 0..16 {
            hash_byte ^= rope[(block*16)+index];
        }
        hash += &(format!("{:02x}", hash_byte));
    }

    hash
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_empty() {
        assert_eq!("a2582a3a0e66e6e86e3812dcb672a272", hash(String::from("")));
    }

    #[test]
    fn test_hash_sequence() {
        assert_eq!("63960835bcdc130f0b66d7ff4f6a5a8e", hash(String::from("1,2,4")));
    }
}

fn get_cursor(bitmap: &Vec<Vec<(u8, u32)>>) -> Option<(usize, usize)>{

    for (row_index, row) in bitmap.iter().enumerate() {
        for (col_index, position) in row.iter().enumerate() {
            if position.1 == 0 && position.0 == 1 {
                return Some((row_index, col_index));
            }
        }
    }
    None
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let hash_input: String = args.get(1).unwrap().clone();

    // Generate each of the hash inputs
    let hash_inputs: Vec<String> = (0..128).map(|row| {
        format!("{}-{}", hash_input, row)
    }).collect();

    // Generate a knot hash for each input
    let hashes: Vec<String> = hash_inputs.iter().map(|hash_input| {
        hash(hash_input.clone())
    }).collect();
    
    // Convert each knot hash into binary
    let vector_map: Vec<Vec<Vec<u8>>> = hashes.iter().map(|hash| {
       hash.chars().map(|value| to_binary(value)).collect()
    }).collect();

    let mut bitmap: Vec<Vec<(u8, u32)>> = Vec::new();

    // Take the sum of each row.
    let mut map_sum: u32 = 0;
    for row in vector_map {
        let mut row_vector: Vec<(u8, u32)> = Vec::new();
        for column in row {
            for digit in column {
                map_sum += digit as u32;
                row_vector.push((digit, 0_u32));
            }
        }
        bitmap.push(row_vector);
    }

    println!("{:?}", bitmap);

    // Perform a search for clusters. Oof. Here's my plan. Randomly select a
    // location, perform a search of Von Neuman space, adding neighbors. Continue
    // to perform this loop until there are no new neighbors. Randomly select a
    // new unmapped location.
    let mut groups = 0;

    loop {
        let cursor = match get_cursor(&bitmap) {
            Some(coordinate) => coordinate,
            None => break
        };

        groups += 1;

        let mut positions_of_interest: Vec<(usize, usize)> = Vec::new();
        positions_of_interest.push(cursor);

        loop {
            let mut new_positions_of_interest: Vec<(usize, usize)> = Vec::new(); 
            for position_of_interest in positions_of_interest {
                let mut neighbors: Vec<(usize, usize)> = [(0, 0), (-1, 0), (0, -1), (0, 1), (1, 0)].iter()
                                     .map(|c| (c.0 as i32 + position_of_interest.0 as i32, c.1 as i32 + position_of_interest.1 as i32))
                                     .filter(|c| c.0 >= 0 && c.0 < 128 &&  c.1 >= 0 && c.1 < 128)
                                     .map(|c| (c.0 as usize, c.1 as usize))
                                     .collect();
                for neighbor_coords in neighbors {
                    let neighbor = bitmap[neighbor_coords.0][neighbor_coords.1];
                    if neighbor.0 == 1 && neighbor.1 == 0 {
                        bitmap[neighbor_coords.0][neighbor_coords.1] = (neighbor.0, groups);
                        new_positions_of_interest.push(neighbor_coords);
                    }
                }
            }
            if new_positions_of_interest.len() == 0 {
                break;
            }
            positions_of_interest = new_positions_of_interest;
        }
    }

    println!("\nThe number of \"on\" cells is {:?}.", map_sum);
    println!("There are {} distinct groups.", groups);
}

use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: String = match args.get(1)  {
        None => panic!("You must supply a file to evaluate."),
        Some(filename) => filename.clone()
    };

    println!("Processing file {:?}", filename);

    let f = File::open(filename).expect("file not found");
    let file = BufReader::new(&f);

    let mut lengths: Vec<usize> = Vec::new();

    for wrapped_line in file.lines() {
        let line = wrapped_line.unwrap();
        let line_lengths = line.split(",").collect::<Vec<&str>>();
        for length in &line_lengths {
            lengths.push(length.parse().unwrap());
        }
    }

    let mut rope: Vec<u32> = Vec::new();

    let rope_size: usize = 256;

    for index in 0..rope_size {
        rope.push(index as u32);
    }

    let mut cursor: usize = 0;
    let mut skip: usize = 0;

    for length in &lengths {
        println!("Cursor: {:?}, Skip: {:?}, Length: {:?}", cursor, skip, length);
        println!("{:?}", rope);

        let mut sublist: Vec<u32> = Vec::new();

        // Get a vector of the elements between cursor and cursor + length.
        if cursor + length >= rope_size {
            let end_1 = rope_size;
            let end_2 = length - (rope_size-cursor);
            sublist.extend_from_slice(&rope.get(cursor..end_1).unwrap());
            sublist.extend_from_slice(&rope.get(0..end_2).unwrap());

        } else {
            sublist.extend_from_slice(rope.get(cursor..(cursor+length)).unwrap());
        }
        println!("{:?}", sublist);

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
        cursor += length + skip;

        // Do a quick cursor boundary check.
        while cursor >= rope.len() {
            cursor -= rope.len();
        }

        // Increment the skip size.
        skip += 1;

        println!("{:?}", rope);
    }

    println!("{:?}", rope[0] * rope[1]);
}

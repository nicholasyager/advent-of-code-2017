use std::env;
use std::fs::File;
use std::io::Read;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: String = match args.get(1)  {
        None => panic!("You must supply a file to evaluate."),
        Some(filename) => filename.clone()
    };

    println!("Processing file {:?}", filename);

    let mut f = File::open(filename).expect("file not found");
    let mut file = BufReader::new(&f);

    let mut checksum: u32 = 0;

    for line in file.lines() {
        let mut values: Vec<u32> = Vec::new();
        for number_value in line.unwrap().split_whitespace().collect::<Vec<&str>>() {
            values.push(number_value.parse().unwrap());
        }

        let mut division_found = false;
        for value_1 in &values {
            for value_2 in &values {
                if value_1 == value_2 {
                    continue
                }
                if value_1 % value_2 == 0 {
                    println!("{:?} % {:?} = 0", value_1, value_2);
                    checksum += value_1 / value_2;
                    division_found = true;
                    break
                }
            }
            if division_found {
                break;
            }
        }

        
    }

    println!("Checksum: {:?}", checksum);

}

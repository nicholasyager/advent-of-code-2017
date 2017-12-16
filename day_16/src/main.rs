use std::env;
use std::fs::File;
use std::io::Read;

/*
    Spin: Move size characters from the end to the front while maintaining order.
 */
fn spin(vector: &Vec<char>, size: usize) -> Vec<char> {
    let mut new_vector = Vec::new();
    let mut swapping_chars = Vec::new();

    for index in (vector.len() - size)..vector.len() {
        swapping_chars.push(vector[index]);
    }

    for character in swapping_chars {
        new_vector.push(character);
    }
    for index in 0..(vector.len()-size) {
        new_vector.push(vector[index]);
    }
    new_vector
}

/*
    Exchange: Make the characters at positions A and B swap places.
 */
fn exchange(vector: &Vec<char>, a: usize, b: usize) -> Vec<char> {
    let mut new_vector = vector.clone();
    new_vector.swap(a, b);
    new_vector
}

/*
    Partner: Make the characters NAMED A and B swap places.
 */
fn partner(vector: &Vec<char>, a: char, b: char) -> Vec<char> {
    let mut new_vector = vector.clone();
    let mut a_index = 0;
    let mut b_index = 0;
    for (index, character) in new_vector.iter().enumerate() {
        if *character == a {
            a_index = index;
        } else if *character == b {
            b_index = index;
        }
    }
    new_vector.swap(a_index, b_index);
    new_vector
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spinning() {
        let mut vector: Vec<char> = vec!['a', 'b', 'c', 'd', 'e'];
        assert_eq!(vec!['d', 'e', 'a', 'b', 'c'], spin(&vector, 2));
    }

    #[test]
    fn test_exchange() {
        let  mut vector: Vec<char> = vec!['a', 'b', 'c', 'd', 'e'];
        assert_eq!(vec!['a', 'b', 'c', 'e', 'd'], exchange(&vector, 3, 4));
    }

    #[test]
    fn test_partner() {
        let  mut vector: Vec<char> = vec!['a', 'b', 'c', 'd', 'e'];
        assert_eq!(vec!['a', 'b', 'c', 'e', 'd'], partner(&vector, 'e', 'd'));
    }
}

fn vector_to_string(vector: Vec<char>) -> String {
    vector.iter().fold(String::from(""), |vec_string, &c| {
                let mut vec_clone = vec_string.clone();
                vec_clone.push(c);
                vec_clone
            })
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: String = args.get(1).unwrap().clone();
    let dances: usize = args.get(2).unwrap().parse().unwrap();

    let mut file = String::new();
    let mut f = File::open(filename).expect("file not found");
    f.read_to_string(&mut file).expect("something went wrong reading the file");

    let commands: Vec<String> = file.trim().split(",").map(|s| {s.to_string()}).collect();

    let mut vector: Vec<char> = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h',
                                     'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p'];   

    println!("{:?}", vector);

    let mut dances_seen: Vec<Vec<char>> = Vec::new();

    for dance_number in 0..dances {

        // Check for cycles. If we've seen this before, then compute the remainder.
        if dances_seen.contains(&vector) {
            println!("{:?}", vector_to_string(dances_seen[dances % dance_number].clone()));
            return
        } else {
            dances_seen.push(vector.clone());
        }

        for command_string in &commands {
            let new_vector = match command_string.chars().nth(0).unwrap() {
                's' => {
                    let length: usize = command_string[1..].parse().unwrap();
                    spin(&vector, length)
                }
                'x' => {
                    let parts: Vec<&str> = command_string[1..].split("/").collect();
                    let coordinates: Vec<usize> = parts.iter().map(|c| String::from(*c).parse().unwrap()).collect();
                    exchange(&vector, coordinates[0], coordinates[1])
                },
                'p' => {
                    let parts: Vec<&str> = command_string[1..].split("/").collect();
                    let chars: Vec<char> = parts.iter().map(|c| String::from(*c).parse().unwrap()).collect();
                    partner(&vector, chars[0], chars[1])
                },
                _ => {vector.clone()}
            };
            vector = new_vector;  
        }
        let vector_string = vector_to_string(vector.clone());
        println!("{:20} - {:?}", dance_number, vector_string);    
    }
   
}

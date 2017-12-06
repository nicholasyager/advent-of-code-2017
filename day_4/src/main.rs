use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashSet;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: String = match args.get(1)  {
        None => panic!("You must supply a file to evaluate."),
        Some(filename) => filename.clone()
    };

    println!("Processing file {:?}", filename);

    let f = File::open(filename).expect("file not found");
    let file = BufReader::new(&f);

    let mut number_of_valid_passphrases: u32 = 0;

    for line in file.lines() {

        let mut words: HashSet<String> = HashSet::new();
        let mut anagram_descriptions: Vec<HashSet<(char, u32)>> = Vec::new();

        let mut valid = true;
        for word in line.unwrap().split_whitespace().collect::<Vec<&str>>() {
            if !words.contains(word) {
                words.insert(String::from(word).to_lowercase());
            } else {
                valid = false;
                break;
            }

            // Compute a tuple hashset
            let mut letter_counts: HashMap<char, u32> = HashMap::new();
            let mut anagram: HashSet<(char, u32)> = HashSet::new();

            for letter in String::from(word).to_lowercase().chars() {
                if letter_counts.contains_key(&letter) {
                    let current_count = letter_counts.get(&letter).unwrap().clone();
                    letter_counts.insert(letter, current_count + 1);
                } else {
                    letter_counts.insert(letter, 1);
                }
            }

            for (letter, count) in &letter_counts {
                anagram.insert((letter.clone(), count.clone()));
            }

            // Check if this word is a permutation of a pre-existing word.
            //for anagram in anagram_descriptions. If not, add it to the list.
            for existing_anagram in &anagram_descriptions {
                if anagram == *existing_anagram {
                    valid = false;
                    break;
                }
            }

            if valid {
                anagram_descriptions.push(anagram);
            }
        }

        if valid {
            number_of_valid_passphrases += 1;
        }

    }
    println!("Found {:?} valid passphrases.", number_of_valid_passphrases);
}
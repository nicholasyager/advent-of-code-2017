use std::env;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::iter::Iterator;
use std::ops::Sub;
use std::ops::Add;
use std::ops::AddAssign;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Coordinate {
    x: i32,
    y: i32
}

impl Add for Coordinate {
    type Output = Coordinate;
    fn add(self, other: Coordinate) -> Coordinate {
        Coordinate {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl Sub for Coordinate {
    type Output = Coordinate;
    fn sub(self, other: Coordinate) -> Coordinate {
        Coordinate {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }
}

impl AddAssign for Coordinate {
    fn add_assign(&mut self, other: Coordinate) {
        self.x += other.x;
        self.y += other.y;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: String = args.get(1).unwrap().clone();

    let mut route_map: HashMap<Coordinate, char> = HashMap::new();

    println!("Processing file {:?}", filename);

    let f = File::open(filename).expect("file not found");
    let file = BufReader::new(&f);

    let mut index: Coordinate = Coordinate{x:0, y:0};
    let mut direction: Coordinate = Coordinate{x:0, y:1};

    for (y, wrapped_line) in file.lines().enumerate() {
        let line = wrapped_line.unwrap();
        for (x, character) in line.chars().enumerate() {
            if character != ' ' {
                route_map.insert(Coordinate{x: x as i32, y: y as i32}, character);
            }

            // Store the starting index if we've found it.
            if y == 0 && character == '|' {
                index = Coordinate{x: x as i32, y: y as i32}
            }
        }
    }

    let mut path_string: String = String::from("");
    let mut steps = 0;

    loop {

        // Process the current character
        let current_character = match route_map.get(&index) {
            Some(character) => character,
            None => break
        };

        direction = match *current_character {
            '+' => {
                // Find the next direction.
                if (direction == Coordinate{x: 0, y:1}) ||  (direction == Coordinate{x: 0, y: -1}) {
                    // Look from side to side
                    if route_map.contains_key(&Coordinate{x: index.x-1, y: index.y}) {
                        Coordinate{x: -1, y: 0}
                    } else {
                        Coordinate{x: 1, y: 0}
                    }

                } else {
                    // Look up and down.
                    if route_map.contains_key(&Coordinate{x: index.x, y: index.y-1}) {
                        Coordinate{x: 0, y: -1}
                    } else {
                        Coordinate{x: 0, y: 1}
                    }
                }
            },
            '|' => direction,
            '-' => direction,
            _ => {
                path_string.push(current_character.clone());
                direction
            }
        };

        // Determine the next index
        index += direction;
        steps += 1;
    }

    println!("Route completed in {} steps.", steps);
    println!("Reconstructed path: {:?}", path_string);

}
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

#[derive(Clone, Debug)]
struct Hex {
    x: i32,
    y: i32
}

impl Hex {
    fn distance(&self, other_hex: Hex) -> i32 {
        ((self.x - other_hex.x).abs() + (self.x + self.y - other_hex.x - other_hex.y).abs() + (self.y - other_hex.y).abs()) / 2
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: String = match args.get(1)  {
        None => panic!("You must supply a file to evaluate."),
        Some(filename) => filename.clone()
    };

    println!("Processing file {:?}", filename);

    let f = File::open(filename).expect("file not found");
    let file = BufReader::new(&f);


    let mut child_position = Hex{x: 0_i32, y: 0_i32};

    for wrapped_line in file.lines() {
        let line = wrapped_line.unwrap();
        let parts = line.split(",").collect::<Vec<&str>>();
        for part in &parts {
            match *part {
                "n" => child_position.y -= 1,
                "ne" => {
                    child_position.x += 1;
                    child_position.y -= 1;
                },
                "se" => child_position.x += 1,
                "s" => child_position.y += 1,
                "sw" => {
                    child_position.x -= 1;
                    child_position.y += 1;
                },
                "nw" => child_position.x -= 1,
                _ => panic!("Unknown direction found!")
            }
        }
    }
    println!("{:?}", child_position);

    let distance = child_position.distance(Hex{x: 0, y: 0});
    println!("Distance from origin: {:?}.", distance);
}
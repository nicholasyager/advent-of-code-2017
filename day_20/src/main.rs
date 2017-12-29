extern crate regex;

use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::iter::Iterator;

use std::collections::HashSet;
use std::collections::HashMap;

use regex::Regex;
use regex::Captures;

use std::ops::Sub;
use std::ops::Add;
use std::ops::AddAssign;


#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Coordinate {
    x: i64,
    y: i64,
    z: i64
}

impl Add for Coordinate {
    type Output = Coordinate;
    fn add(self, other: Coordinate) -> Coordinate {
        Coordinate {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl Sub for Coordinate {
    type Output = Coordinate;
    fn sub(self, other: Coordinate) -> Coordinate {
        Coordinate {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

impl AddAssign for Coordinate {
    fn add_assign(&mut self, other: Coordinate) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

#[derive(Debug, Clone)]
struct Particle {
    coordinate: Coordinate,
    velocity: Coordinate,
    acceleration: Coordinate
}

impl Particle {
    fn tick(&mut self) {
        self.velocity += self.acceleration;
        self.coordinate += self.velocity;
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: String = args.get(1).unwrap().clone();

    // Here's my theory. We can approximate the particles as a second-order
    // polynomial of the form Px = X + Vx * t + Ax * t^2. We store these values
    // for each coordinate and use this to predict each coordinate at some
    // arbitrarily large time step. This is more complicated than relying on
    // the accelerations alone, but I do not know if part 2 will require knowing
    // more about the actual particles that we are "simulating."

    let mut particles: Vec<Particle> = Vec::new();
    let evaluation_time: i64 = 1000;

    let mut minimum_distance: u32 = 100000000;
    let mut closest_particle: usize = 0;

    // Parse out the particles
    let f = File::open(filename).expect("file not found");
    let file = BufReader::new(&f);
    let re = Regex::new(r"[pav]=<([\-0-9]{1,10}),([\-0-9]{1,10}),([\-0-9]{1,10})>").unwrap();
    for (index, wrapped_line) in file.lines().enumerate() {
        let line = wrapped_line.unwrap();
        let parts = line.split(", ").collect::<Vec<&str>>();
        let mut particle: Vec<(i64, i64, i64)> = Vec::new();
        for part in parts {
            let captures = re.captures(part).unwrap();
            let coordinate: (i64, i64, i64) = (
                captures.get(1).unwrap().as_str().parse().unwrap(),
                captures.get(2).unwrap().as_str().parse().unwrap(),
                captures.get(3).unwrap().as_str().parse().unwrap()
            );
            particle.push(coordinate);
           
        }
        particles.push(Particle{
            coordinate: Coordinate{x: particle[0].0, y: particle[0].1, z: particle[0].2},
            velocity: Coordinate{x: particle[1].0, y: particle[1].1, z: particle[1].2},
            acceleration: Coordinate{x: particle[2].0, y: particle[2].1, z: particle[2].2}
        });

        // Calculate the future distance.
        let mut distance = (particle[0].0 + (particle[1].0 * evaluation_time) + (particle[2].0 * evaluation_time.pow(2))).abs() as u32; 
        distance += (particle[0].1 + (particle[1].1 * evaluation_time) + (particle[2].1 * evaluation_time.pow(2))).abs() as u32; 
        distance += (particle[0].2 + (particle[1].2 * evaluation_time) + (particle[2].2 * evaluation_time.pow(2))).abs() as u32; 
        
        if distance < minimum_distance {
            minimum_distance = distance;
            closest_particle = index;
        }
    }
    println!("The closet particle is particle {} ({}).", closest_particle, minimum_distance);

    // Simulate collisions for some arbitrary period of time.
    let mut ignore_list: HashSet<usize> = HashSet::new();
    for time in 0..1000 {
        let mut coordinates: HashMap<Coordinate, Vec<usize>> = HashMap::new();

        for index1 in 0..particles.len() {
            if ignore_list.contains(&index1) {
                continue;
            }

            let mut particle = particles[index1].clone();
            particle.tick();
            particles[index1] = particle.clone();
            let position = particle.coordinate;

            let mut index_list: Vec<usize> = Vec::new();
            if coordinates.contains_key(&position) {
                index_list = coordinates.get(&position).unwrap().clone();
            }
            index_list.push(index1);
            coordinates.insert(position, index_list);
        }

        for (position, indices) in coordinates {
            if indices.len() > 1 {
                for index in indices {
                    ignore_list.insert(index);
                }
            }
        }
        println!("Time {}: {} removed, {} remain.", time, ignore_list.len(), particles.len() - ignore_list.len());
    }
}

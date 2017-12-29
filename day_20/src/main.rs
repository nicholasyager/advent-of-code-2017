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

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: String = args.get(1).unwrap().clone();

    // Here's my theory. We can approximate the particles as a second-order
    // polynomial of the form Px = X + Vx * t + Ax * t^2. We store these values
    // for each coordinate and use this to predict each coordinate at some
    // arbitrarily large time step. This is more complicated than relying on
    // the accelerations alone, but I do not know if part 2 will require knowing
    // more about the actual particles that we are "simulating."

    let mut particles: Vec<Vec<(i64, i64, i64)>> = Vec::new();
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
        particles.push(particle.clone());

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
    for time in 0..10000 {
        let mut coordinates: HashMap<(i64, i64, i64), Vec<usize>> = HashMap::new();

        for index1 in 0..particles.len() {
            if ignore_list.contains(&index1) {
                continue;
            }

            let position =  (particles[index1][0].0 + (particles[index1][1].0 * time) + (particles[index1][2].0 * time.pow(2)),
                             particles[index1][0].1 + (particles[index1][1].1 * time) + (particles[index1][2].1 * time.pow(2)),
                             particles[index1][0].2 + (particles[index1][1].2 * time) + (particles[index1][2].2 * time.pow(2))); 

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
    }
    println!("{} removed, {} remain.", ignore_list.len(), particles.len() - ignore_list.len());
}

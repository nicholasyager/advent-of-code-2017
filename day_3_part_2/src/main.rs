use std::env;

fn index_from_coords(coordinates: (i32, i32), width: u32) -> i32 {
    let n: i32 = width as i32;
    (coordinates.0+(coordinates.1*n)) as i32
}


fn main() {
    let args: Vec<String> = env::args().collect();

    let index: u32 = match args.get(1)  {
        None => panic!("You must supply an index to evaluate."),
        Some(argument) => {
            argument.parse().unwrap()
        }
    };

    // Part two doesn't allow for a clever solution :-( I'll start by
    // constructing the necessary vector or vectors. Then I will start to fill
    // in the values just like in the algorithm.

    // Find the ring number. Each ring has 8*n elements in it, where n is the
    // ring number.

    let mut total: u32 = 1;
    let mut ring: u32 = 0;
    while total < index {
        ring += 1;
        total += 8 * ring;
    }

    // The world must be at most as large as the total of the last ring.
    let mut world: Vec<u32> = Vec::with_capacity(total as usize);
    for _ in 0..total as usize {
        world.push(0);
    }

    let width = (ring*2) + 1;

    // Start at the middle point.
    let mut coordinate = (ring as i32, ring as i32);
    let mut value = 1;

    // Save the initial value
    world[index_from_coords(coordinate, width) as usize] = value;

    let mut side = 3;
    let directions: [(i32, i32); 4] = [(0, -1), (-1, 0), (0, 1), (1, 0)];
    let neighbors: [(i32, i32); 8] = [(-1, -1), (0, -1), (1, -1), 
                                      (-1,  0),          (1,  0),
                                      (-1,  1), (0,  1), (1,  1)];
    let mut steps = 1;
    let mut current_ring = 0;

    loop {
    
        // Check if the ring is done. If not calculate some new coordinates.
        let side_length = (current_ring * 8) / 4;

        if steps >= side_length {
            steps = 0;
            side += 1;
        }

        if side >= 4 {

            if ring == current_ring {
                break
            }

            current_ring += 1;
            steps = 0;
            side = 0;
            coordinate.0 += 1;
        } else {

            // Find the new coordinate
            let direction = directions[side];
            coordinate = ((coordinate.0 as i32 + direction.0), 
                            (coordinate.1 as i32 + direction.1));
        }

        // Calculate the sum of the neighbor values
        value = 0;
        for neighbor in &neighbors {

            let new_coordinate = ((coordinate.0 as i32 + neighbor.0) as i32,
                                              (coordinate.1 as i32 + neighbor.1) as i32);

            let read_index: i32 = index_from_coords(new_coordinate, width);

            if new_coordinate.0 < 0 || new_coordinate.0 >= width as i32 || new_coordinate.1 < 0 || new_coordinate.1 >= width as i32 {
                    continue
            }

            value += world[read_index as usize];
        }

        let write_index = index_from_coords(coordinate, width) as usize;
        world[write_index] = value;
        steps += 1;

        if value > index {
            println!("The value {:?} is the first value greater than {:?}.", value, index);
            break
        }
    }

}

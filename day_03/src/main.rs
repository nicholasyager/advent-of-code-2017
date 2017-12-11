use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();

    let index: u32 = match args.get(1)  {
        None => panic!("You must supply an index to evaluate."),
        Some(argument) => {
            argument.parse().unwrap()
        }
    };
    println!("{:?}", index);

    // Clever Solution Time!
    // Here's my clever solution for day 3. Using the index, find the ring that
    // the index is located in. This will be one of the coordinates in the
    // distance calculation. Then, find out how far the index is from the middle
    // of the side of the ring. This is the other coordinate.

    // Find the ring number. Each ring has 8*n elements in it, where n is the
    // ring number.

    let mut total: u32 = 1;
    let mut ring: u32 = 0;
    while total < index {
        ring += 1;
        total += 8 * ring;
    }
    println!("The index {:?} is located on ring {:?}.", index, ring);

    // Determine how far the index is from the middle of its side. This will be
    // done by subtracting side_length from the index until the remaining 
    // quantity is < side_length. Then, find (side_length / 2) + 1. This is the
    // mid-point. Subtract the remaining index from the mod-point.

    let ring_size = 8*ring;
    let side_length: u32 = ring_size / 4;
    println!("Each side has {:?} elements.", side_length);

    let distance_from_ring_end = total - index;

    let mut distance_from_side_end = distance_from_ring_end;
    while distance_from_side_end > side_length {
        distance_from_side_end -= side_length;
    }

     println!("The index {:?} is {:?} units from the end of its side.", index, 
        distance_from_side_end);

    let middle_point = side_length / 2;
    let distance_from_middle_point;
    if middle_point > distance_from_side_end {
         distance_from_middle_point= middle_point - distance_from_side_end;
    } else {
         distance_from_middle_point = distance_from_side_end - middle_point;
    }

    println!("The index {:?} is {:?} units from the middle point.", index,
        distance_from_middle_point);

    // The distance will then be the sum of the two side of the triangle. Tada!
    let distance = ring + distance_from_middle_point;
    println!("The index {:?} is {:?} units from the center.", index, distance);
}

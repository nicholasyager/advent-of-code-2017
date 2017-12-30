use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::iter::Iterator;

use std::collections::HashMap;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Image {
    width: usize,
    height: usize,
    values: Vec<bool>
}

fn coordinate_to_index(x: usize, y: usize, w: usize) -> usize {
    x + (y * w)
}


impl Image {

    fn split(&self) -> Vec<Image> {
        let mut images: Vec<Image> = Vec::new();
        let image_width: usize;

        if self.width % 2 == 0 {
            image_width = 2;
        } else {
            image_width = 3;
        }

        let total_edge_length = self.width/image_width;
        let total_images = (self.width/image_width).pow(2);

        for image_index in 0..total_images {
            // println!("Image index: {}", image_index);
            let mut image_values: Vec<bool> = Vec::new();
            let mut row_skip: usize = 0;
            for row in 0..image_width {
                for col in 0..image_width {

                    let padded_row = row + ((image_index / total_edge_length)*image_width);
                    let padded_col = col + ((image_index % total_edge_length)*image_width);
                    let index = coordinate_to_index(padded_col, padded_row, self.width);

                    // println!("{} + {}*{} = {}", padded_col, padded_row, image_width, index);
                    image_values.push(self.values[index]);
                }
            }
            images.push(Image{width: image_width, height: image_width, values: image_values});
        }
        images
    }

    fn test_rule(&self, rule: Vec<bool>) -> bool {
        // Test rotations
        let mut rotation_image = self.clone();
        for _ in 0..4 {
            rotation_image = rotation_image.rotate();

            if rule == rotation_image.values {
                return true
            }
              // Flip rows
            let flipped_image = rotation_image.flip_row();
            if rule == flipped_image.values {
                return true
            }

            // Flip Columns
            let flipped_image = rotation_image.flip_col();
            if rule == flipped_image.values {
                return true
            }

            // Flip both
            let flipped_image = rotation_image.flip_row().flip_col();
            if rule == flipped_image.values {
                return true
            }
        }
        false
    }

    fn flip_row(&self) -> Image {
        let mut image_values: Vec<bool> = Vec::new();
        for row in 0..self.width {
            for col in (0..self.width).rev() {
                image_values.push(self.values[(row*self.height as usize) +col]);
            }
        }
        Image{width: self.width, height: self.width, values: image_values}
    }

    fn flip_col(&self) -> Image {
        let mut image_values: Vec<bool> = Vec::new();
        for row in (0..self.width).rev() {
            for col in 0..self.width {
                image_values.push(self.values[(row*self.height as usize) +col]);
            }
        }
        Image{width: self.width, height: self.width, values: image_values}
    }

    fn rotate(&self) -> Image {
        let mut image_values: Vec<bool> = Vec::new();
        for col in 0..self.width {
            for row in (0..self.width).rev() {
                image_values.push(self.values[(row*self.height as usize) + col]);
            }
        }
        Image{width: self.width, height: self.width, values: image_values}
    }

    fn print(&self) {
        for row in 0..self.width {
            for col in 0..self.width {
                let value = self.values[(row*self.height as usize) + col];
                if value {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            print!("\n");
        }
        print!("\n");
    }
}

fn convert_rule(rule: Vec<bool>) -> String {
    let mut rule_string = String::from("");
    for element in rule {
        if element {
            rule_string.push('#');
        } else {
            rule_string.push('.');
        }
    }
    rule_string
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: String = args.get(1).unwrap().clone();
    let iterations: usize = args.get(2).unwrap().parse().unwrap();

    // Load the rule list.
    let mut rules: HashMap<Vec<bool>, Vec<bool>> = HashMap::new();

    let f = File::open(filename).expect("file not found");
    let file = BufReader::new(&f);

    for wrapped_line in file.lines() {
        let line = wrapped_line.unwrap();
        let parts = line.split(" => ").collect::<Vec<&str>>();
        let key = parts[0].replace("/","").chars().map(|c| {
            match c {
                '.' => false,
                '#' => true,
                _ => panic!("Oh noes!")
            }
        }).collect::<Vec<bool>>();

        let value = parts[1].replace("/","").chars().map(|c| {
            match c {
                '.' => false,
                '#' => true,
                _ => panic!("Oh noes!")
            }
        }).collect::<Vec<bool>>();
        rules.insert(key, value);
        
    }
    println!("Loaded {} rules.", rules.len());

    // Create the initial image.
    let mut image: Image = Image{width: 3, height: 3, 
                                 values: vec![false, true, false, 
                                              false, false, true, 
                                              true, true, true]};
    image.print();

    for iteration in 0..iterations {

        // Get a list of images to mosaic
        let image_pieces = image.split();

        let mut new_images_pieces: Vec<Image> = Vec::new();

        let mut piece_cache: HashMap<Image, Image> = HashMap::new();

        for (image_part_index, image_piece) in image_pieces.iter().enumerate() {

            if piece_cache.contains_key(image_piece) {
                new_images_pieces.push(piece_cache.get(image_piece).unwrap().clone());
                continue;
            } else {
                // Evaluate the enhancement rules against each image_piece. If the
                // rule matches, then save a new image pieces.
                for (rule, replacement) in &rules {
                    if image_piece.test_rule(rule.clone()) {
                        let new_image_piece: Image = Image{width: image_piece.width + 1,
                                                     height: image_piece.width + 1,
                                                     values: replacement.clone()};
                        piece_cache.insert(image_piece.clone(), new_image_piece.clone());
                        new_images_pieces.push(new_image_piece);
                        break;
                    }
                }
            }
        }

        // Stitch together the mosaic into an image. To do this, we need to find
        // the square root of the number of elements to identify how to 
        // construct the mosaic. Once the edge length has been determined, we
        // want to interlace the existing images to stitch. This can be done
        // by constructing each row of the mosaic at a time.
        let edge_length = (new_images_pieces.len() as f64).sqrt() as usize;
        let mut mosaic_image_values: Vec<bool> = Vec::new();
        
        let image_piece_height = new_images_pieces[0].height;

        // For each row in the mosaic, we want to interlace the rows of the
        // image pieces. This will involve keeping track of the max length of
        // each image.
        for mosaic_row in 0..edge_length {

            // Determine the images to load in the row.
            let images = new_images_pieces.get((mosaic_row*edge_length)..((mosaic_row+1)*edge_length)).unwrap();

            for mosaic_row_height in 0..image_piece_height {
                for image in images {
                    for col in 0..image.width {
                        let value = image.values[(mosaic_row_height*image.height as usize) + col];
                        mosaic_image_values.push(value);
                    }
                }
            }
        }
        image = Image{width: (mosaic_image_values.len() as f64).sqrt() as usize, 
                      height: (mosaic_image_values.len() as f64).sqrt() as usize, 
                      values: mosaic_image_values.clone()};

        let mut total_on = 0;
        for value in image.values.clone() {
            if value {
                total_on += 1
            }
        }

        println!("There are {} pixels on after {} iterations.", total_on, iteration+1);

    }
}

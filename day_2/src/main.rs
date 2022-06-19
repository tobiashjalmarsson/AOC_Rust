
use std::io::{BufRead, BufReader};
use std::fs::File;



struct Location {
    depth : i32,
    forward : i32
}

impl Location {
    fn display(&self) {
        println!("Depth is : {}, Forward is: {}", self.depth, self.forward);
    }
}

fn main() {
    let mut location = Location  {
        depth: 0,
        forward: 0
    };

    let file: File = File::open("inputs.txt").expect("Unable to open file inputs.txt");
    let reader: BufReader<File> = BufReader::new(file);
    for line in reader.lines() {
        let current_line = line.expect("Error reading string");
        let (direction, quantity) = split_input(&current_line);
        location = update_location(direction, quantity, location);
    }

    location.display();
}



fn update_location(direction : &str, amount : i32, current_location : Location) -> Location {
    let mut new_location : Location = current_location;
    match direction {
        "up" => new_location.depth = new_location.depth - amount,
        "down" => new_location.depth = new_location.depth + amount,
        _other => new_location.forward = new_location.forward + amount

    }

    new_location
} 

fn split_input(input_string : &String) -> (&str, i32) {
    let arr = input_string.as_bytes();
    let mut space_idx: usize = 0;
    for (idx, &elem) in arr.iter().enumerate(){
        if elem == b' '{
            space_idx = idx;
        }
    }
    (&input_string[..space_idx], 2)
}
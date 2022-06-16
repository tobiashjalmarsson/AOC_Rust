use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    let file = File::open("inputs.txt").expect("Unable to open file");
    let reader: BufReader<File> = BufReader::new(file);
    
    let mut bigger_than: u16 = 0;
    let mut _previous_value = 0;
    let mut _current_value = 0;
    let mut _previous_value = 0;
    for (idx, line) in reader.lines().enumerate() {
        let mut _line: String = line.expect("Unable to read line");
        
        if idx == 0 {
            _current_value = _line.trim().parse().expect("Cant parse");
        } else {
            _previous_value = _current_value;
            _current_value = _line.trim().parse().clone().expect("Cant parse");
            if _current_value > _previous_value {
                bigger_than = bigger_than + 1;
            }

        }
    }


    println!("Number of increases: {}", bigger_than);
}  

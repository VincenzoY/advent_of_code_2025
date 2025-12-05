use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("../../input.txt") {
        let mut zeros_count: i32 = 0;
        let mut dial_position: i32 = 50;
        for line in lines.map_while(Result::ok) {
            let direction = &line[0..1];
            let distance = &line[1..].parse::<i32>().unwrap();
            
            let new_dial_position = if direction == "L" { dial_position - distance } else { dial_position + distance };
            let new_dial_position = (new_dial_position + 100) % 100;

            if new_dial_position == 0 { zeros_count += 1 };
            dial_position = new_dial_position;
        }

        println!("{zeros_count}");
    }
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
use std::io::{self, BufRead};
use std::path::Path;
use std::fs::File;

fn main() {
    let mut sum = 0;
    if let Ok(lines) = read_lines("../../input.txt") {
        for line in lines.map_while(Result::ok) {
            let mut cur_max = 0;
            //let mut stack: Vec<u32> = vec![];
            let mut last_largest_digit: Option<u32> = None;
            for char in line.chars().rev() {
                let digit = char.to_digit(10).unwrap();

                match last_largest_digit {
                    Some(val) => {
                        let cur_value = digit * 10 + val;
                        if cur_value > cur_max {
                            cur_max = cur_value;
                        }
                    },
                    None => {
                        last_largest_digit = Some(digit)
                    }
                }

                if digit > last_largest_digit.expect("Should not happend") {
                    last_largest_digit = Some(digit);
                }
            }
            println!("{}", cur_max);
            sum += cur_max;
        }
    }

    println!("{}", sum);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
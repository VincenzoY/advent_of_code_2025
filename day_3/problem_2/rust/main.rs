use std::io::{self, BufRead};
use std::path::Path;
use std::fs::File;
use std::cmp::max;

fn main() {
    let mut sum = 0;
    if let Ok(lines) = read_lines("../../input.txt") {
        for line in lines.map_while(Result::ok) {
            let length = line.len();
            let mut memoize: Vec<[Option<u64>; 12]> = vec![[None; 12]; length];
            for (line_index, char) in line.chars().rev().enumerate() {
                let digit: u64 = char.to_digit(10).unwrap().into();
                if line_index == 0 {
                    memoize[0][0] = Some(digit);
                    continue;
                }

                let last_line = memoize[line_index - 1];

                for digit_index in 0..12 {
                    if digit_index > 0 && last_line[digit_index - 1] == None {
                        break;
                    }

                    let value_including_cur_digit: u64 = {
                        if digit_index == 0 {
                            digit
                        } else {
                            last_line[digit_index - 1].expect("Value should be defined") + digit * 10_u64.pow(digit_index as u32)
                        }
                    };

                    let value_excluding_cur_digit = last_line[digit_index];

                    memoize[line_index][digit_index] = match value_excluding_cur_digit {
                        Some(value_excluding_cur_digit) => {
                            Some(max(
                                value_including_cur_digit,
                                value_excluding_cur_digit
                            ))
                        },
                        None => {
                            Some(value_including_cur_digit)
                        }
                    }
                }
            }
            let largest_value = memoize[length - 1][11].unwrap();
            //println!("{:?}", largest_value);
            sum += largest_value;
        }
    }

    println!("{}", sum);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
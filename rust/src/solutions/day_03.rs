use crate::solution::{Answer, Solution};
use std::{cmp::max, error::Error};

pub struct Day03Solution {}

impl Solution<u64> for Day03Solution {
    fn problem_one(input: &str) -> Result<Answer, Box<dyn Error>>  {
        let mut sum: u64 = 0;

        for line in input.lines() {
            let mut cur_max: u64 = 0;
            let mut last_largest_digit: Option<u64> = None;

            for char in line.chars().rev() {
                let digit = char.to_digit(10).unwrap() as u64;

                match last_largest_digit {
                    Some(val) => {
                        let cur_value: u64 = digit * 10 + val;
                        if cur_value > cur_max {
                            cur_max = cur_value;
                        }
                    },
                    None => {
                        last_largest_digit = Some(digit)
                    }
                }

                if digit > last_largest_digit.expect("Should not happen") {
                    last_largest_digit = Some(digit);
                }
            }

            sum += cur_max;
        }

        Ok(Answer::Number(sum))
    }

    fn problem_two(input: &str) -> Result<Answer, Box<dyn Error>>  {
        let mut sum = 0;

        for line in input.lines() {
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
            sum += largest_value;
        }

        Ok(Answer::Number(sum))
    }
}

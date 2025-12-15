use crate::solution::{Answer, Solution};
use std::error::Error;

pub struct Day06Solution {}

impl Solution for Day06Solution {
    fn problem_one(input: &str) -> Result<Answer, Box<dyn Error>>  {
        let mut lines_iter = input.lines().rev();

        let operators: Vec<&str> = lines_iter.next().unwrap().split_whitespace().collect();
        let mut values: Vec<i64> = vec![0; operators.len()];

        for (index, value) in values.iter_mut().enumerate() {
            *value = match operators[index] {
                "*" => 1,
                "+" => 0,
                _ => panic!("Invalid Op"),
            }
        }

        for line in lines_iter {
            for (index, value) in line.split_whitespace().enumerate() {
                let value = value.parse::<i64>().unwrap();
                
                match operators[index] {
                    "*" => { values[index] *= value; },
                    "+" => { values[index] += value; },
                    _ => panic!("Invalid Op"),
                }
            }
        }

        Ok(Answer::Integer(values.iter().sum()))
    }

    fn problem_two(input: &str) -> Result<Answer, Box<dyn Error>>  {
        let operators: Vec<&str> = input.lines().rev().next().unwrap().split_whitespace().collect();

        let mut values: Vec<i64> = vec![0; operators.len()];
        for (index, value) in values.iter_mut().enumerate() {
            *value = match operators[index] {
                "*" => 1,
                "+" => 0,
                _ => panic!("Invalid Op"),
            }
        }

        let mut column_values: Vec<i64> = vec![0; input.lines().next().unwrap().len()];

        for line in input.lines() {
            let chars: Vec<char> = line.chars().collect();
            let mut operator_idx = 0;
            if chars[0] == '*' || chars[0] == '+' {
                for char_index in 0..chars.len() {
                    let next_char = if char_index + 1 < chars.len() { Some(chars[char_index + 1])} else { None };
                    if next_char == Some('*') || next_char == Some('+') { 
                        operator_idx += 1; 
                        continue; 
                    }

                    match operators[operator_idx] {
                        "*" => { values[operator_idx] *= column_values[char_index]; },
                        "+" => { values[operator_idx] += column_values[char_index]; },
                        _ => panic!("Missing operator")
                    }
                }
            } else {
                for (char_index, char) in chars.iter().enumerate() {
                    if *char == ' ' { continue; }
                    let digit = char.to_digit(10).unwrap() as i64;
                    column_values[char_index] = column_values[char_index] * 10 + digit;
                }
            }
        }

        Ok(Answer::Integer(values.iter().sum()))
    }
}

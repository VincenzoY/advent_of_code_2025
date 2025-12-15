use std::error::Error;

use crate::solution::{Answer, Solution};

pub struct Day01Solution {}

impl Solution<i64> for Day01Solution {
    fn problem_one(input: &str) -> Result<Answer, Box<dyn Error>> {
        let lines = input.lines();
        let mut zeros_count: i64 = 0;
        let mut dial_position: i64 = 50;
        for line in lines {
            let direction = &line[0..1];
            let distance = &line[1..].parse::<i64>().unwrap();
            
            let new_dial_position = if direction == "L" { dial_position - distance } else { dial_position + distance };
            let new_dial_position = (new_dial_position + 100) % 100;

            if new_dial_position == 0 { zeros_count += 1 };
            dial_position = new_dial_position;
        }

       Ok(Answer::Integer(zeros_count))
    }

    fn problem_two(input: &str) -> Result<Answer, Box<dyn Error>> {
        let lines = input.lines();
        let mut zeros_count: i64 = 0;
        let mut dial_position: i64 = 50;
        for line in lines {
            let direction = &line[0..1];
            let distance = &line[1..].parse::<i64>().unwrap();

            let full_spins = distance / 100;
            let distance = distance - full_spins * 100;
            zeros_count += full_spins;
            
            let new_dial_position = if direction == "L" { dial_position - distance } else { dial_position + distance };
            if (new_dial_position <= 0 || new_dial_position >= 100) && dial_position != 0 { zeros_count += 1 };

            let new_dial_position = (new_dial_position + 100) % 100;
            dial_position = new_dial_position;
        }

        Ok(Answer::Integer(zeros_count))
    }
}
use std::error::Error;

use crate::solution::{Answer, Solution};

pub struct Day07Solution {}

impl Solution for Day07Solution {
    fn problem_one(input: &str) -> Result<Answer, Box<dyn Error>> {
        let mut lines_iter = input.lines().into_iter();

        let initial_line = lines_iter.next().unwrap();
        let mut cur_state = vec![0; initial_line.len()];
        let mut next_state = vec![0; initial_line.len()];
        let mut split_count = 0;

        for (index, char) in initial_line.chars().enumerate() {
            if char == 'S' { cur_state[index] = 1; }
        }

        for line in lines_iter {
            let chars: Vec<char> = line.chars().collect();
            for index in 0..chars.len() {
                let char = chars[index];

                if cur_state[index] != 1 { continue; }

                if char == '^' {
                    split_count += 1;
                    if chars[index - 1] == '.' { next_state[index - 1] = 1; }
                    if chars[index + 1] == '.' { next_state[index + 1] = 1; }
                } else {
                    next_state[index] = 1;
                }
            }

            cur_state = next_state.clone();
            next_state = vec![0; initial_line.len()];
        }

        Ok(Answer::Integer(split_count))
    }

    fn problem_two(input: &str) -> Result<Answer, Box<dyn Error>> {
        let mut lines_iter = input.lines().into_iter();

        let initial_line = lines_iter.next().unwrap();
        let mut state: Vec<Vec<char>> = vec![vec!['.'; initial_line.len()]];

        for (index, char) in initial_line.chars().enumerate() {
            if char == 'S' { state[0][index] = '|'; }
        }

        for line in lines_iter {
            let chars: Vec<char> = line.chars().collect();

            let cur_state = &state[state.len() - 1];
            let mut next_state = vec!['.'; initial_line.len()];
            for index in 0..chars.len() {
                let char = chars[index];

                if char == '^' {
                    next_state[index] = '^';

                    if cur_state[index] == '|' {
                        if chars[index - 1] == '.' { next_state[index - 1] = '|'; }
                        if chars[index + 1] == '.' { next_state[index + 1] = '|'; }
                    }
                } else if cur_state[index] == '|' {
                    next_state[index] = '|';
                }
            }
            state.push(next_state);
        }

        let mut combination_counts = vec![0; initial_line.len()];
        let mut next_combination_counts = vec![0; initial_line.len()];

        for (index, char) in state.last().unwrap().iter().enumerate() {
            if *char == '|' { combination_counts[index] = 1; }
        }

        for level_index in (0..state.len() - 1).rev() {
            let line = &state[level_index];
            
            for (index, char) in line.iter().enumerate() {
                if *char == '|' {
                    if state[level_index + 1][index] == '^' { 
                        next_combination_counts[index] = 
                            combination_counts[index - 1] + 
                            combination_counts[index + 1];
                    } else {
                        next_combination_counts[index] = combination_counts[index];
                    }
                }
            }
            combination_counts = next_combination_counts.clone();
            next_combination_counts = vec![0; initial_line.len()];
        }

        Ok(Answer::Integer(combination_counts.iter().sum()))
    }
}
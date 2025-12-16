use crate::solution::{Answer, Solution};
use std::{convert::TryInto, error::Error};

const ADJACENT_DIRECTIONS: [[i32; 2]; 8] = [
    [-1, -1],
    [-1,  0],
    [-1,  1],
    [ 0, -1],
    [ 0,  1],
    [ 1, -1],
    [ 1,  0],
    [ 1,  1],
];

pub struct Day04Solution {}

impl Solution for Day04Solution {
    fn problem_one(input: &str) -> Result<Answer, Box<dyn Error>>  {
        let matrix = parse_input(input);
        let mut result = 0;

        for y in 0..matrix.len() {
            for x in 0..matrix[0].len() {
                let y: i32 = y as i32;
                let x: i32 = x as i32;
                if let Some('@') = matrix_value(&matrix, (y, x)) {
                    let neighbours = {
                        let mut sum = 0;

                        for [delta_y, delta_x] in ADJACENT_DIRECTIONS {
                            if let Some('@') = matrix_value(&matrix, (y + delta_y, x + delta_x)) {
                                sum += 1;
                            }
                        }

                        sum
                    };

                    if neighbours < 4 {
                        result += 1;
                    }
                }
            }
        }

        Ok(Answer::Integer(result))
    }

    fn problem_two(input: &str) -> Result<Answer, Box<dyn Error>>  {
        let mut matrix = parse_input(input);
        let mut result = 0;
        let mut has_made_change = true;

        while has_made_change {
            has_made_change = false;
            for y in 0..matrix.len() {
                for x in 0..matrix[0].len() {
                    let y: i32 = y as i32;
                    let x: i32 = x as i32;
                    if let Some('@') = matrix_value(&matrix, (y, x)) {
                        let neighbours = {
                            let mut sum = 0;

                            for [delta_y, delta_x] in ADJACENT_DIRECTIONS {
                                if let Some('@') = matrix_value(&matrix, (y + delta_y, x + delta_x)) {
                                    sum += 1;
                                }
                            }

                            sum
                        };

                        if neighbours < 4 {
                            result += 1;
                            matrix[y as usize][x as usize] = '.';
                            has_made_change = true;
                        }
                    }
                }
            }
        }

        Ok(Answer::Integer(result))
    }
}

fn matrix_value(matrix: &Vec<Vec<char>>, (y, x): (i32, i32)) -> Option<char> {
    let y_len = matrix.len().try_into().unwrap();
    let x_len = matrix[0].len().try_into().unwrap();

    if x < 0 || x >= x_len || y < 0 || y >= y_len {
        return None;
    }

    let value = matrix[y as usize][x as usize];

    match value {
        '@' => Some('@'),
        _ => None
    }
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    let mut matrix: Vec<Vec<char>> = vec![];

    for line in input.lines() {
        matrix.push(line.chars().collect());
    }

    return matrix;
}

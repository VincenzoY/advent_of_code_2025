use std::error::Error;

use crate::solution::{Answer, Solution};

pub struct GenericSolution {}

impl Solution for GenericSolution {
    fn problem_one(input: &str) -> Result<Answer, Box<dyn Error>> {
        Err("Unimplemented".into())
    }

    fn problem_two(input: &str) -> Result<Answer, Box<dyn Error>> {
        Err("Unimplemented".into())
    }
}
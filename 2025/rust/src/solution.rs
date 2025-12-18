use std::error::Error;

#[derive(Debug)]
pub enum Answer {
    Number(u64),
    Integer(i64),
    String(String),
}

pub trait Solution {
    fn problem_one(input: &str) -> Result<Answer, Box<dyn Error>>;
    fn problem_two(input: &str) -> Result<Answer, Box<dyn Error>>;
}

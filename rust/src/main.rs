use std::env;
use std::process;
use std::fs;
use aoc::solution::Solution;
use aoc::solutions::*;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    let file_name = if config.testing {
        format!("./tests/day_{}.txt", config.day)
    } else {
        format!("./day_{}.txt", config.day)
    };

    let file_name = format!("../inputs/{file_name}");
    let file = fs::read_to_string(file_name).unwrap();

    let (problem_one_answer, problem_two_answer) = match config.day {
        1 => (Day01Solution::problem_one(&file), Day01Solution::problem_two(&file)),
        2 => (Day02Solution::problem_one(&file), Day02Solution::problem_two(&file)),
        3 => (Day03Solution::problem_one(&file), Day03Solution::problem_two(&file)),
        4 => (Day04Solution::problem_one(&file), Day04Solution::problem_two(&file)),
        5 => (Day05Solution::problem_one(&file), Day05Solution::problem_two(&file)),
        _ => process::exit(1),
    };

    println!("For day {}{}", config.day, if config.testing {" - TESTING"} else {""});
    match problem_one_answer {
        Ok(answer) =>  {
            println!("Problem 1: {answer:?}");
        }
        Err(err) => {
            println!("Problem 1: Error in processing ({err:?})");
        }
    }
    match problem_two_answer {
        Ok(answer) =>  {
            println!("Problem 2: {answer:?}");
        }
        Err(err) => {
            println!("Problem 2: Error in processing ({err:?})");
        }
    }
}

struct Config {
    day: i64,
    testing: bool,
}

impl Config {
    fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let day = match args.next() {
            Some(day_str) => {
                let day = day_str.parse::<i64>()
                    .map_err(|_| "Specified day is not a number.")?;

                if day <= 0 || day > 14 {
                    return Err("Invalid day specified.");
                }

                day
            },
            None => { return Err("Missing date specification."); }
        };

        let testing = env::var("TESTING").is_ok();
        

        Ok(Config{ day, testing })
    }
}


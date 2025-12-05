use crate::solution::{Answer, Solution};
use std::{cmp::max, error::Error};

pub struct Day05Solution {}

impl Solution<u64> for Day05Solution {
    fn problem_one(input: &str) -> Result<Answer, Box<dyn Error>>  {
        let (mut ranges, ingredients) = parse_input(input);
        ranges.sort_by_key(|range| range.0);

        let condensed_ranges = condense_ranges(ranges);
        let mut fresh = 0;

        for ingredient in ingredients {
            for range in &condensed_ranges {
                // would be faster to binary search rather than lin search
                if range.0 > ingredient {
                    continue;
                }

                if range.0 > ingredient {
                    break;
                }

                if range.0 <= ingredient && ingredient <= range.1 {
                    fresh += 1;
                }
            }
        }

        Ok(Answer::Number(fresh))
    }

    fn problem_two(input: &str) -> Result<Answer, Box<dyn Error>>  {
        let (mut ranges, _) = parse_input(input);
        ranges.sort_by_key(|range| range.0);

        let condensed_ranges = condense_ranges(ranges);
        let mut total = 0;

        for range in &condensed_ranges {
            total += range.1 - range.0 + 1;
        }

        Ok(Answer::Number(total))
    }
}

fn condense_ranges(ranges: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    let mut condensed_ranges: Vec<(u64, u64)> = vec![];

    condensed_ranges.push(ranges[0]);
    for index in 1..ranges.len() {
        let range = ranges[index];
        let last_index = condensed_ranges.len() - 1;
        let last_condensed_range = condensed_ranges[last_index];

        if range.0 <= last_condensed_range.1 {
            condensed_ranges[last_index] = (
                last_condensed_range.0,
                max(last_condensed_range.1, range.1)
            );
        } else {
            condensed_ranges.push(range);
        }
    }

    condensed_ranges
}

fn parse_input(input: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let mut ranges: Vec<(u64, u64)> = vec![];
    let mut ingredients: Vec<u64> = vec![];
    let mut lines = input.lines();

    while let Some(line) = lines.next() {
        if line == "" { break; }

        if let [start, end] = line.split("-").collect::<Vec<&str>>()[..] {
            ranges.push((start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap()));
        }
    }

    while let Some(line) = lines.next() {
        ingredients.push(line.parse::<u64>().unwrap());
    }

    return (ranges, ingredients);
}

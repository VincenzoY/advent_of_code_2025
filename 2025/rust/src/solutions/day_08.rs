use std::error::Error;
use std::env;

use crate::solution::{Answer, Solution};

pub struct Day08Solution {}

#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
    z: i64
}

impl Point {
    fn distance(&self, point: &Point) -> i64 {
        (point.x - self.x).pow(2) +
        (point.y - self.y).pow(2) + 
        (point.z - self.z).pow(2)
    }

    fn new(x: i64, y: i64, z: i64) -> Point {
        Point { x, y, z }
    }
}

impl Solution for Day08Solution {
    fn problem_one(input: &str) -> Result<Answer, Box<dyn Error>> {
        let max_connections = if env::var("TESTING").is_ok() { 10 } else { 1000 };

        let mut points: Vec<Point> = vec![];

        for line in input.lines() {
            let raw_point: Vec<&str> = line.split(",").collect();
            let point = Point::new(
                raw_point[0].parse().unwrap(),
                raw_point[1].parse().unwrap(),
                raw_point[2].parse().unwrap()
            );
            points.push(point);
        }

        let points_len = points.len();
        let mut point_group: Vec<usize> = (0..points_len).collect();
        let mut distances: Vec<(i64, usize, usize)> = vec![];

        for first_point_idx in 0..points_len - 1 {
            for second_point_idx in first_point_idx + 1..points_len {
                let first_point = &points[first_point_idx];
                let second_point = &points[second_point_idx];
                distances.push((
                    first_point.distance(second_point),
                    first_point_idx,
                    second_point_idx
                ))
            }
        }

        distances.sort_by_key(|x| x.0);

        for (_, first_point_idx, second_point_idx) in distances.iter().take(max_connections) {
            let first_point_group = point_group[*first_point_idx];
            let second_point_group = point_group[*second_point_idx] ;
            if first_point_group == second_point_group { continue; }

            point_group.iter_mut().for_each(|group| if *group == second_point_group { *group = first_point_group; } );
        }

        let mut group_count = vec![0; points_len];

        for group in point_group {
            group_count[group] += 1;
        }
        group_count.sort_by_key(|x| -*x);

        Ok(Answer::Integer(group_count[0] * group_count[1] * group_count[2]))
    }

    fn problem_two(input: &str) -> Result<Answer, Box<dyn Error>> {
        let mut points: Vec<Point> = vec![];

        for line in input.lines() {
            let raw_point: Vec<&str> = line.split(",").collect();
            let point = Point::new(
                raw_point[0].parse().unwrap(),
                raw_point[1].parse().unwrap(),
                raw_point[2].parse().unwrap()
            );
            points.push(point);
        }

        let points_len = points.len();
        let mut point_group: Vec<usize> = (0..points_len).collect();
        let mut distances: Vec<(i64, usize, usize)> = vec![];

        for first_point_idx in 0..points_len - 1 {
            for second_point_idx in first_point_idx + 1..points_len {
                let first_point = &points[first_point_idx];
                let second_point = &points[second_point_idx];
                distances.push((
                    first_point.distance(second_point),
                    first_point_idx,
                    second_point_idx
                ))
            }
        }

        distances.sort_by_key(|x| x.0);

        let mut connections = 0;
        let mut distance_index = 0;
        let last_points = loop {
            let (_, first_point_idx, second_point_idx) = distances[distance_index];
            distance_index += 1;   

            let first_point_group = point_group[first_point_idx];
            let second_point_group = point_group[second_point_idx] ;
            if first_point_group == second_point_group { continue; }

            point_group.iter_mut().for_each(|group| if *group == second_point_group { *group = first_point_group; } );
            connections += 1;

            if connections == points_len - 1 { break (first_point_idx, second_point_idx); }
        };

        Ok(Answer::Integer(points[last_points.0].x * points[last_points.1].x))
    }
}
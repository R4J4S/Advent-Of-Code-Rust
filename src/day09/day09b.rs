use itertools::{Itertools, Position};
use num::abs;

pub fn solve(input: String) -> i64 {
    input
        .lines()
        .map(|line| {
            let mut input_vector = line
                .split_whitespace()
                .map(|num_string| num_string.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            input_vector.reverse();

            let mut end_vector = Vec::<i64>::new();
            loop {
                if input_vector.iter().all(|n| n == &0) {
                    break;
                }

                input_vector = input_vector
                    .iter()
                    .tuple_windows::<(&i64, &i64)>()
                    .with_position()
                    .map(|(position, (left, right))| {
                        let diff = left - right;

                        match position {
                            Position::Last | Position::Only => {
                                end_vector.push(*right);
                            }
                            _ => {}
                        }

                        diff
                    })
                    .collect::<Vec<i64>>();
            }

            end_vector.reverse();
            // println!("END:{:?}", &end_vector);
            end_vector.iter().fold(0, |acc, value| (value - acc))
        })
        .fold(0, |acc, value| (acc + value))
}

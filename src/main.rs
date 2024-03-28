mod Day01;
mod Utils;

use std::{env, fs};

use crate::Utils::grid;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_path: &str = &args[1];

    let file_content = fs::read_to_string(&input_path).expect("Failed to read input");
    let result = solve(file_content);
    println!("RESULT:{}", &result);
}

pub fn solve(input: String) -> i32 {
    let mut grid = grid::Grid::new(input);
    dbg!(grid);

    0
}

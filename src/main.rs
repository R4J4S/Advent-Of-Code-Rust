use std::{env, fs};

mod day07;
fn main() {
    let args: Vec<String> = env::args().collect();
    let input_path: &str = &args[1];

    let file_content = fs::read_to_string(&input_path).expect("Failed to read input");
    let result = solve_day(file_content);
    println!("RESULT:{}", &result);
    assert_eq!(result, 71503);
}

pub fn solve_day(input: String) -> u64 {
    day07::day07a::solve(input)
}

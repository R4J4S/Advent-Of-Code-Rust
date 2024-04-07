use std::{env, fs};

mod day10;
fn main() {
    let args: Vec<String> = env::args().collect();
    let input_path: &str = &args[1];

    let file_content = fs::read_to_string(&input_path).expect("Failed to read input");
    let result = solve_day(file_content);
    println!("RESULT:{}", &result);
    println!("REQUIRED:{}", 2);
}

pub fn solve_day(input: String) -> u64 {
    day10::day10a::solve(input)
}

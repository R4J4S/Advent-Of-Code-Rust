use std::{
    collections::{HashMap, HashSet},
    env, fs,
};
fn main() {
    let args: Vec<String> = env::args().collect();
    let input_path: &str = &args[1];

    let file_content = fs::read_to_string(&input_path).expect("Failed to read input");
    let result = solve_day(file_content);
    assert_eq!(result, 30);
    println!("RESULT:{}", &result);
}

pub fn solve_day(input: String) -> u32 {
    solve(input)
}

pub fn solve(input: String) -> u32 {
    0
}

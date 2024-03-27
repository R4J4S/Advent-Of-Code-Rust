use std::{cmp, env, fs};

mod Day01;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_path: &str = &args[1];

    let file_content = fs::read_to_string(&input_path).expect("Failed to read input");
    let result = solve(file_content);
    println!("RESULT:{}", &result);
}

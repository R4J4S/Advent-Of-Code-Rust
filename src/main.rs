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
    let lines = input.lines();
    let mut card_counts: Vec<u32> = vec![1; lines.count() + 1];
    card_counts[0] = 0;

    input
        .lines()
        .map(|line| -> i32 {
            println!("================================================================");
            println!("{}", &line);

            let numbers_string = line.split_once(":").unwrap();
            let current_id = numbers_string
                .0
                .split_whitespace()
                .last()
                .unwrap()
                .parse::<i32>()
                .unwrap();

            let (winning_numbers_string, my_number_string) =
                numbers_string.1.split_once("|").unwrap();

            let winning_numbers_set: HashSet<i32> = winning_numbers_string
                .trim()
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<HashSet<i32>>();

            let mut cards_won = 0;
            my_number_string
                .trim()
                .split_whitespace()
                .map(|s| {
                    let num = s.parse::<i32>().unwrap();

                    if winning_numbers_set.contains(&num) {
                        cards_won += 1;
                    }
                })
                .for_each(drop);

            //part 2 ================================================================================
            let current_cards = card_counts[current_id as usize];
            for i in 1..cards_won + 1 {
                let suceeding_id = current_id + i;
                if suceeding_id < card_counts.len() as i32 {
                    card_counts[suceeding_id as usize] += 1 * current_cards;
                }
            }
            // print_vector(&card_counts);
            0
        })
        .for_each(drop);

    // dbg!(&card_counts);
    let answer: u32 = card_counts.iter().sum();
    answer
}

fn print_hashmap<K, V>(hashmap: &HashMap<K, V>)
where
    K: std::fmt::Debug + std::hash::Hash + Eq,
    V: std::fmt::Debug,
{
    for (key, value) in hashmap {
        println!("{:?}: {:?}", key, value);
    }
}

fn print_vector<T>(vector: &[T])
where
    T: std::fmt::Debug,
{
    for (index, value) in vector.iter().enumerate() {
        println!("{}: {:?}", index, value);
    }
}

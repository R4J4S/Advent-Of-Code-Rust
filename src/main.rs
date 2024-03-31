use std::{env, fs};
fn main() {
    let args: Vec<String> = env::args().collect();
    let input_path: &str = &args[1];

    let file_content = fs::read_to_string(&input_path).expect("Failed to read input");
    let result = solve_day(file_content);
    println!("RESULT:{}", &result);
    assert_eq!(result, 46);
}

pub fn solve_day(input: String) -> u64 {
    solve(input)
}
#[derive(Debug)]
pub struct MapRangeData {
    pub destination: u64,
    pub source: u64,
    pub range: u64,
}

impl MapRangeData {
    pub fn new(destination: u64, source: u64, range: u64) -> MapRangeData {
        MapRangeData {
            destination,
            source,
            range,
        }
    }

    pub fn get_range_value(&self, number: u64) -> (u64, bool) {
        if number >= self.source && number < self.source + self.range {
            let offset = number - self.source;
            (self.destination + offset, true)
        } else {
            (number, false)
        }
    }
}

pub fn solve(input: String) -> u64 {
    let mut lowest_seed_location = u64::MAX;
    let input_splits: Vec<&str> = input.split("\n\n").collect();

    let seeds_range = get_seeds_range(input_splits[0]);
    let mut seeds_vector = (seeds_range.0..seeds_range.1).collect::<Vec<u64>>();

    input_splits.iter().skip(1).for_each(|map_str| {
        let map_range_datas: Vec<MapRangeData> = map_str
            .lines()
            .skip(1)
            .map(|line| {
                let v: Vec<u64> = line
                    .trim()
                    .split_whitespace()
                    .map(|number_string| number_string.parse::<u64>().unwrap())
                    .collect();

                MapRangeData::new(v[0], v[1], v[2])
            })
            .collect();

        for seed_index in 0..seeds_vector.len() {
            for (i, map_range_data) in map_range_datas.iter().enumerate() {
                let (value, is_value_changed) =
                    map_range_data.get_range_value(seeds_vector[seed_index]);

                //Last iteration
                if (i == map_range_datas.len() - 1) && value < lowest_seed_location && value != 0 {
                    lowest_seed_location = value;
                }

                if is_value_changed {
                    seeds_vector[seed_index] = value;
                    break;
                }
            }
        }
    });

    lowest_seed_location
}

pub fn get_seeds_range(seed_string: &str) -> (u64, u64) {
    let seeds_input = seed_string
        .split_once(": ")
        .unwrap()
        .1
        .trim()
        .split_whitespace()
        .map(|number_string| number_string.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let seed_chunks = seeds_input.chunks(2).collect::<Vec<&[u64]>>();
    let mut seed_range = (u64::MAX, u64::MIN);
    for seed_chunk in seed_chunks {
        let (start, end) = (seed_chunk[0], seed_chunk[0] + seed_chunk[1]);

        if start <= seed_range.0 {
            seed_range.0 = start;
        }

        if end >= seed_range.1 {
            seed_range.1 = end;
        }
    }

    seed_range
}

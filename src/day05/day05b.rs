/*
BRUTE FORCE SOLUTION
This solution is super slow even with rayon took around 3 minutes
TODO: OPTIMISE LATER
*/

use rayon::iter::*;

#[derive(Debug)]
pub struct RangeData {
    pub destination_start: u64,
    pub destination_end: u64,
    pub source_start: u64,
    pub source_end: u64,
    pub range: u64,
}

impl RangeData {
    pub fn new(destination: u64, source: u64, range: u64) -> RangeData {
        RangeData {
            destination_start: destination,
            source_start: source,
            destination_end: destination + range,
            source_end: source + range,
            range: range,
        }
    }

    pub fn get_range_value(&self, number: u64) -> (u64, bool) {
        if number >= self.source_start && number < self.source_end {
            let offset = number - self.source_start;
            (self.destination_start + offset, true)
        } else {
            (number, false)
        }
    }
}

pub fn solve(input: String) -> u64 {
    let input_splits: Vec<&str> = input.split("\n\n").collect();

    println!("############################################################################################################");
    println!("MAP RANGE DATAS PARSING STARTED");
    let all_map_datas = input_splits
        .iter()
        .skip(1)
        .map(|map_str| {
            map_str
                .lines()
                .skip(1)
                .map(|line| {
                    let v: Vec<u64> = line
                        .trim()
                        .split_whitespace()
                        .map(|number_string| number_string.parse::<u64>().unwrap())
                        .collect();

                    RangeData::new(v[0], v[1], v[2])
                })
                .collect::<Vec<RangeData>>()
        })
        .collect::<Vec<Vec<RangeData>>>();

    // print_vector(&all_map_datas, "PARSED MAP RANGE DATAS");
    println!("MAP RANGE DATAS PARSING COMPLETE");
    println!("############################################################################################################\n");

    println!("############################################################################################################");
    println!("SEEDS RANGE PARSING STARTED");
    let seed_ranges = get_seeds_ranges(input_splits[0]);
    let seed_progress_bar =
        indicatif::ProgressBar::new(seed_ranges.iter().map(|(start, end)| end - start).sum());

    let mut seed_vector = Vec::new();

    let seeds: Vec<_> = seed_ranges
        .par_iter()
        .flat_map(|(start, end)| {
            let mut range_seeds = Vec::with_capacity((*end - *start) as usize);
            for seed in *start..*end {
                range_seeds.push(seed);
                seed_progress_bar.inc(1);
            }
            range_seeds
        })
        .collect();

    seed_vector.extend(seeds);

    // println!("SEED SET: {:#?}", &seed_set);
    println!("SEEDS RANGE PARSING COMPLETE");
    println!("############################################################################################################\n");

    println!("############################################################################################################");
    println!("SEEDS RANGE MANIPULATION STARTED");

    let locations_progress_bar = indicatif::ProgressBar::new(seed_vector.len() as u64);
    let locations = seed_vector
        .par_iter()
        .map(|seed| {
            let mut seed_value = *seed;

            all_map_datas.iter().for_each(|map_data| {
                for conversion_map in map_data {
                    let range_value = conversion_map.get_range_value(seed_value);
                    seed_value = range_value.0;
                    if range_value.1 {
                        break;
                    }
                }
            });

            locations_progress_bar.inc(1);
            seed_value
        })
        .collect::<Vec<u64>>();

    println!("SEEDS RANGE MANIPULATION COMPLETE");
    println!("############################################################################################################\n");

    *locations.iter().min().unwrap()
}

pub fn get_seeds_ranges(seed_string: &str) -> Vec<(u64, u64)> {
    let seeds_input = seed_string
        .split_once(": ")
        .unwrap()
        .1
        .trim()
        .split_whitespace()
        .map(|number_string| number_string.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let mut seed_ranges = Vec::with_capacity(seeds_input.len() / 2);

    let mut i = 0;
    while i < seeds_input.len() {
        seed_ranges.push((seeds_input[i], seeds_input[i] + seeds_input[i + 1]));
        i += 2;
    }

    seed_ranges
}

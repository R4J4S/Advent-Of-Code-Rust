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
    let input_splits: Vec<&str> = input.split("\n\n").collect();

    let mut seeds: Vec<u64> = get_seeds(input_splits[0]);
    println!("SEEDS INITIAL {:?}\n", &seeds);

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

        for seed_index in 0..seeds.len() {
            for (_i, map_range_data) in map_range_datas.iter().enumerate() {
                println!("{:?}", &map_range_data);
                let (value, is_value_changed) = map_range_data.get_range_value(seeds[seed_index]);
                seeds[seed_index] = value;
                if is_value_changed {
                    println!("SEEDS UPDATED {:?}", &seeds);
                    break;
                }
            }
        }

        println!("################################################\n");
    });

    println!("SEEDS FINAL {:?}", &seeds);

    *seeds.iter().min().unwrap()
}

pub fn get_seeds(seed_string: &str) -> Vec<u64> {
    seed_string
        .split_once(": ")
        .unwrap()
        .1
        .trim()
        .split_whitespace()
        .map(|number_string| number_string.parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
}

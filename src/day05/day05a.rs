pub fn solve(input: String) -> u64 {
    let mut input_split = input.split("\n\n");

    let mut seeds = get_seeds(&input_split.next().unwrap());

    for (i, map_string) in input_split.into_iter().enumerate() {
        let now = Instant::now();

        let conversion_map = get_conversion_map(map_string);
        for seed_index in 0..seeds.len() {
            if let Some(&value) = conversion_map.get(&seeds[seed_index]) {
                seeds[seed_index] = value;
            }
        }

        let elapsed = now.elapsed().as_millis();

        println!("MAP {} : COMPLETED Time Taken:{:.2?}ms", &i, &elapsed); // ====================================================================================", &i);
    }

    *seeds.iter().min().unwrap() as u64
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

pub fn get_conversion_map(map_string: &str) -> HashMap<u64, u64> {
    let mut resultant_map: HashMap<u64, u64> = HashMap::new();

    map_string.lines().skip(1).for_each(|line| {
        let v: Vec<u64> = line
            .trim()
            .split_whitespace()
            .map(|number_string| number_string.parse::<u64>().unwrap())
            .collect();

        let (destination, source, range) = (v[0], v[1], v[2]);

        //println!("D:{}, S:{}, R:{}", &destination, &source, &range);

        let mut counter = 0;
        for _i in source..(source + range) {
            resultant_map.insert(source + counter, destination + counter);
            counter += 1;
        }
    });

    resultant_map
}

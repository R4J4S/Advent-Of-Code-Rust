pub fn solve(input: String) -> u64 {
    //parsing input
    let v = parse_input(&input);
    let data_length = v.len() / 2;
    let (time_data, distance_data) = v.split_at(data_length);

    let answer = (0..data_length)
        .map(|index| {
            let (time, distance) = (time_data[index], distance_data[index]);
            (0..time)
                .map(|t| if t * (time - t) > distance { 1 } else { 0 })
                .fold(0, |acc, value| (acc + value))
        })
        .fold(1, |answer, value| (answer * value));

    answer
}

pub fn parse_input(input: &str) -> Vec<u64> {
    input
        .lines()
        .flat_map(|line| {
            line.split_whitespace()
                .skip(1)
                .map(|element| element.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<u64>>()
}

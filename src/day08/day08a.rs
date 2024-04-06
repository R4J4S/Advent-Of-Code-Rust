use std::collections::HashMap;

pub fn parse_navigation_string(input: &str) -> Vec<u8> {
    input
        .chars()
        .map(|c| match c {
            'L' => 0,
            'R' => 1,
            _ => 2,
        })
        .collect::<Vec<u8>>()
}

pub fn parse_navigation_map_string(input: &str) -> HashMap<String, Vec<String>> {
    let mut result = HashMap::<String, Vec<String>>::new();

    input
        .lines()
        .map(|line| {
            let line_split = line.split_once(" = ").unwrap();

            let filtered_split = line_split.1.replace(&['(', ')', ','][..], "");
            let nav_split = filtered_split
                .split(" ")
                .map(|s| s.to_string())
                .collect::<Vec<String>>();

            result.insert(line_split.0.to_string(), nav_split);
        })
        .for_each(drop);

    result
}

pub fn solve(input: String) -> u64 {
    let input_split = input.split("\n\n").collect::<Vec<&str>>();

    let (nav_string, nav_map_string) = (input_split[0], input_split[1]);
    let sample_answer = input_split[2].parse::<u64>().unwrap();

    let navigation_vector = parse_navigation_string(nav_string);
    println!("{:?}", &navigation_vector);

    let navigation_map = parse_navigation_map_string(nav_map_string);
    // println!("{:#?}", &navigation_map);

    let mut step_count: u64 = 0;
    let mut nav_current = String::from("AAA");
    let nav_destination: String = String::from("ZZZ");

    while !nav_current.eq(&nav_destination) {
        for move_choice in &navigation_vector {
            let choices = navigation_map.get(&nav_current).unwrap();
            nav_current = choices[*move_choice as usize].clone();
            step_count += 1;

            println!(
                "MOVE: {} , CHOICE : {} , STEPS: {}",
                &move_choice, &nav_current, &step_count,
            );

            if nav_current.eq(&nav_destination) {
                break;
            }
        }
    }

    assert_eq!(&sample_answer, &step_count);
    step_count
}

//16271

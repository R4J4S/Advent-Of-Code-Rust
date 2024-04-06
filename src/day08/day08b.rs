use std::collections::HashMap;

use num::integer::lcm;

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

pub fn parse_navigation_map_string(input: &str) -> (HashMap<String, Vec<String>>, Vec<String>) {
    let mut result = HashMap::<String, Vec<String>>::new();
    let mut starting_nodes = Vec::<String>::new();

    input
        .lines()
        .map(|line| {
            let line_split = line.split_once(" = ").unwrap();

            let filtered_split = line_split.1.replace(&['(', ')', ','][..], "");
            let nav_split = filtered_split
                .split(" ")
                .map(|s| s.to_string())
                .collect::<Vec<String>>();

            if line_split.0.chars().last().unwrap() == 'A' {
                starting_nodes.push(line_split.0.to_string());
            }
            result.insert(line_split.0.to_string(), nav_split);
        })
        .for_each(drop);

    (result, starting_nodes)
}

pub fn solve(input: String) -> u64 {
    let input_split = input.split("\n\n").collect::<Vec<&str>>();

    let (nav_string, nav_map_string) = (input_split[0], input_split[1]);

    let navigation_vector = parse_navigation_string(nav_string);
    // println!("{:?}", &navigation_vector);

    let (navigation_map, starting_nodes) = parse_navigation_map_string(nav_map_string);
    // println!("{:#?}", &navigation_map);
    println!("STARTING NODES : {:?}", &starting_nodes);

    let mut ghost_step_count: Vec<u64> = vec![0; starting_nodes.len()];

    for (start_node_index, start) in starting_nodes.iter().enumerate() {
        let mut step_count: u64 = 0;
        let mut nav_current = start.clone();
        let mut did_reach_destination = false;

        while !did_reach_destination {
            for move_choice in &navigation_vector {
                let choices = navigation_map.get(&nav_current).unwrap();
                nav_current = choices[*move_choice as usize].clone();
                step_count += 1;

                // println!(
                //     "MOVE: {} , CHOICE : {} , STEPS: {}",
                //     &move_choice, &nav_current, &step_count,
                // );

                did_reach_destination = nav_current.chars().last().unwrap() == 'Z';

                if (did_reach_destination) {
                    break;
                }
            }
        }

        ghost_step_count[start_node_index] = step_count;
    }

    dbg!(&ghost_step_count);

    let answer = ghost_step_count
        .iter()
        .fold(1, |acc, step_count| lcm(acc, *step_count));
    answer
}

//16271

pub fn solve(input: String) -> i32 {
    println!(
        "==================Input==================\n{}\n==================Input==================",
        &input
    );

    let answer = input
        .lines()
        .map(|line| {
            let game_sets = line.split_once(":").unwrap().1;
            println!("GameSet: {}", &game_sets);
            let rgb_max_color_count = game_sets
                .split(";")
                .map(|game_set| {
                    let mut rgb_count = (1, 1, 1);
                    game_set
                        .split(",")
                        .map(|number_and_color| {
                            let split_string =
                                number_and_color.trim().split_once(" ").unwrap_or_default();
                            let color_count = split_string.0.parse::<i32>().unwrap_or(1);

                            match split_string.1 {
                                "red" => rgb_count.0 = color_count,
                                "green" => rgb_count.1 = color_count,
                                "blue" => rgb_count.2 = color_count,
                                _ => (),
                            }
                        })
                        .for_each(drop);
                    rgb_count
                })
                .fold((1, 1, 1), |max_color_counts, color_counts| {
                    (
                        cmp::max(max_color_counts.0, color_counts.0),
                        cmp::max(max_color_counts.1, color_counts.1),
                        cmp::max(max_color_counts.2, color_counts.2),
                    )
                });

            println!(
                "MAX RGB: ({},{},{})",
                &rgb_max_color_count.0, &rgb_max_color_count.1, &rgb_max_color_count.2
            );
            rgb_max_color_count
        })
        .fold(0, |answer, rgb_max_color_count| {
            answer + (rgb_max_color_count.0 * rgb_max_color_count.1 * rgb_max_color_count.2)
        });

    answer
}

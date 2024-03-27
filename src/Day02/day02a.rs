pub fn solve(input: String) {
    const BLUE_COLOR_AVAILABLE: i32 = 14;
    const GREEN_COLOR_AVAILABLE: i32 = 13;
    const RED_COLOR_AVAILABLE: i32 = 12;

    const BLUE_CODE: &str = "blue";
    const RED_CODE: &str = "red";
    const GREEN_CODE: &str = "green";

    println!(
        "==================Input==================\n{}\n==================Input==================",
        &input
    );

    let result = input
        .lines()
        //iterate through line by line
        .map(|line| {
            //Seperate game ID and set strings
            let (game_id, game_sets) = line.split_once(":").unwrap();

            //Set game id
            let id = game_id.replace("Game ", "").parse::<i32>().unwrap();

            //Iterate over all sets
            let is_valid_game = game_sets
                .split(";")
                .map(|set| {
                    //Check if all sets are valid
                    //Iterate over every single set in game
                    let is_valid_set = set
                        .split(",")
                        .map(|number_and_color| {
                            let (number_string, color_code) =
                                number_and_color.trim().split_once(" ").unwrap();
                            let color_count = number_string.parse::<i32>().unwrap_or_default();

                            //Is Valid Set
                            match color_code {
                                RED_CODE => color_count <= RED_COLOR_AVAILABLE,
                                GREEN_CODE => color_count <= GREEN_COLOR_AVAILABLE,
                                BLUE_CODE => color_count <= BLUE_COLOR_AVAILABLE,
                                _ => {
                                    println!("Failed to read color code:{}", &color_code);
                                    false
                                }
                            }
                        })
                        .fold(true, |acc, is_valid_set| (acc && is_valid_set));
                    is_valid_set
                })
                .fold(true, |is_valid_game, is_valid_set| {
                    is_valid_game && is_valid_set
                });

            if is_valid_game {
                id
            } else {
                0
            }
        })
        .fold(0, |sum_of_valid_ids, id| (sum_of_valid_ids + id));

    println!("RESULT:{}", result);
}

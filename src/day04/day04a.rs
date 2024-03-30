pub fn solve(input: String) -> u32 {
    let answer = input
        .lines()
        .map(|line| {
            println!("================================");
            println!("{}", &line);

            let numbers_string = line.split_once(":").unwrap();
            let (winning_numbers_string, my_number_string) =
                numbers_string.1.split_once("|").unwrap();

            println!("WINNING LINE : {}", &winning_numbers_string);
            println!("MY NUMBER LINE : {}", &my_number_string);

            let winning_numbers_set: HashSet<i32> = winning_numbers_string
                .trim()
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<HashSet<i32>>();

            println!("----------------------------------------------");

            let mut win_count = 0;
            my_number_string
                .trim()
                .split_whitespace()
                .map(|s| {
                    let num = s.parse::<i32>().unwrap();

                    if winning_numbers_set.contains(&num) {
                        win_count += 1;
                    }
                })
                .for_each(drop);

            println!("================================");
            win_count
        })
        .fold(0, |acc, win_count| {
            println!("WIN COUNT : {}", &win_count);
            if win_count > 0 {
                acc + 2_i32.pow(win_count - 1)
            } else {
                acc
            }
        }); // (acc + 2_i32.pow(win_count)));
    answer as u32
}

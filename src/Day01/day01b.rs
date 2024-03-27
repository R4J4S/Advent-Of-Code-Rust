pub fn solve(input: String) {
    const NUMBERS: &[&str] = &[
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let lines = input.lines();
    let mut result: i32 = 0;

    for line in lines {
        let mut number_string = String::new();
        for (i, c) in line.chars().into_iter().enumerate() {
            match c {
                '0'..='9' => number_string.push(c),
                'z' | 'o' | 't' | 'f' | 's' | 'e' | 'n' => {
                    for (j, numbered_word) in NUMBERS.iter().enumerate() {
                        let mut end_index = i + numbered_word.len();
                        end_index = end_index.min(line.len());
                        let cmp_string = &line[i..end_index];
                        if &cmp_string == numbered_word {
                            let ch = char::from_digit(j as u32, 10).unwrap();
                            number_string.push(ch);
                        }
                    }
                }
                _ => (),
            }
        }

        if !number_string.is_empty() {
            let mut n_string = String::new();
            n_string.push(number_string.chars().next().unwrap());
            n_string.push(number_string.chars().last().unwrap());

            //parse the number_string into integer
            match n_string.parse::<i32>() {
                //add the integer to into the sum
                Ok(n) => {
                    println!("Adding:{0}", &n);
                    result += n;
                }
                _ => {}
            };
        }
    }

    println!("Result:{0}", &result);
}

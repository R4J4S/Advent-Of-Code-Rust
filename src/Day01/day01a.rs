pub fn solve(input: String) {
    println!("{:?}", &input);

    let mut result: i32 = 0;

    //Paragragh into words
    let lines = input.lines();

    //Loop through chars
    for line in lines {
        //Number string for storing digit chars
        let mut number_string = String::new();
        //loop through chars
        for c in line.chars() {
            //Store char into a string let's say number_string if it's a valid digit
            if c.is_digit(10) {
                number_string.push(c);
            }
        }

        if (!number_string.is_empty()) {
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

    //print the answer
    println!("Result:{0}", &result);
}

use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("No input file");
    let mut sum = 0;
    for line in contents.lines() {
        let mut last_digit = 0;
        let mut found_digit = false;
        for c in line.chars() {
            match c.to_digit(10) {
                Some(digit) => {
                    last_digit = digit;
                    if !found_digit {
                        found_digit = true;
                        sum += 10 * digit;
                    }
                },
                None => {},
            }
        }
        sum += last_digit;
    }
    println!("{sum}");
}

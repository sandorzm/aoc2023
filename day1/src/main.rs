use regex::Captures;
use regex::Regex;
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

    let mut sum = 0;
    let pat = "([0-9]|zero|one|two|three|four|five|six|seven|eight|nine)";
    let re_first = Regex::new(pat).expect("Regex should be valid");
    let re_last = Regex::new(&(".*".to_owned() + pat)).expect("Regex should be valid");
    for line in contents.lines() {
        sum += 10 * parse_digit(re_first.captures(line))
               + parse_digit(re_last.captures(line));
    }
    println!("{sum}");
}

fn parse_digit(c: Option<Captures>) -> u32 {
    match c.expect("Regex should match").get(1).expect("Capture should match").as_str() {
        "zero" => 0,
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        other => other
                 .chars()
                 .next()
                 .expect("Regex should not match empty strings")
                 .to_digit(10)
                 .expect("Regex should only match spelled out numbers or digits"),
    }
}

use regex::Regex;
use std::cmp::max;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Expected input.txt file");
    let re_line = Regex::new("Game (\\d+): (.*)").expect("Line regex should be valid");
    let re_game = Regex::new("(\\d+) (\\w+)").expect("Game regex should be valid");
    let mut sum1 = 0;
    let mut sum2 = 0;
    for line in input.lines() {
        let m = re_line
            .captures(line)
            .expect("Line regex should always match");
        let game_id: u32 = m
            .get(1)
            .expect("Line regex should always match game ID")
            .as_str()
            .parse()
            .expect("Game ID should be integer");
        let game_str = m
            .get(2)
            .expect("Line regex should always match game string")
            .as_str();
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;
        let mut part1_possible = true;
        for m in re_game.captures_iter(game_str) {
            let count: u32 = m
                .get(1)
                .expect("Game regex should always match cube count")
                .as_str()
                .parse()
                .expect("Cube count should be integer");
            let part1_max_count = match m
                .get(2)
                .expect("Game regex should always match cube color")
                .as_str()
            {
                "red" => {
                    max_red = max(max_red, count);
                    12
                }
                "green" => {
                    max_green = max(max_green, count);
                    13
                }
                "blue" => {
                    max_blue = max(max_blue, count);
                    14
                }
                _ => panic!("Cube color should be red, green, or blue"),
            };
            if count > part1_max_count {
                part1_possible = false;
            }
        }
        if part1_possible {
            sum1 += game_id;
        }
        sum2 += max_red * max_green * max_blue;
    }
    println!("{sum1}");
    println!("{sum2}");
}

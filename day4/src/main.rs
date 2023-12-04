use regex::Regex;
use std::collections;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Expected input.txt file");
    let re_line = Regex::new("[^:]*?: ([^|]*?)\\| (.*)").expect("Line regex should be valid");
    let re_num = Regex::new("\\d+").expect("Number regex should be valid");
    let mut sum1 = 0;
    let mut sum2: u32 = 0;
    let mut this_card_instances = 1;
    let mut copies: Vec<(u32, u32)> = vec![]; // Copies carried over from previous cards
    for line in input.lines() {
        sum2 += this_card_instances;
        let m = re_line
            .captures(line)
            .expect("Line regex should always match");
        let winners = m
            .get(1)
            .expect("Line regex should always match winning numbers")
            .as_str();
        let nums = m
            .get(2)
            .expect("Line regex should always match numbers you have")
            .as_str();
        let winners: collections::BTreeSet<u32> = re_num
            .find_iter(winners)
            .map(|m| {
                m.as_str()
                    .parse()
                    .expect("Winning numbers should be integers")
            })
            .collect();
        let mut winners_found = 0;
        for num in re_num.find_iter(nums) {
            if winners.contains(
                &num.as_str()
                    .parse()
                    .expect("Numbers you have should be integers"),
            ) {
                winners_found += 1;
            }
        }
        if winners_found > 0 {
            sum1 += 2_i32.pow(winners_found - 1);
        }
        copies.push((winners_found, this_card_instances));
        this_card_instances = 1; // Next card: reset to original card only, then add copies
        for (winners_found, card_instances) in &mut copies {
            if *winners_found > 0 {
                *winners_found -= 1;
                this_card_instances += *card_instances;
            }
        }
    }
    println!("{sum1}");
    println!("{sum2}");
}

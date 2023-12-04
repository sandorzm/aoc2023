use regex::Regex;
use std::fs;
use std::iter;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Expected input.txt file");
    let mut line1_syms = vec![];
    let mut line1_nums = vec![];
    let mut line2 = "";
    let mut line2_syms = vec![];
    let mut line2_nums: Vec<(usize, usize, u32)> = vec![];
    let re_symbol = Regex::new("[^.\\d]").expect("Symbol regex should be valid");
    let re_number = Regex::new("\\d+").expect("Number regex should be valid");
    let re_star = Regex::new("\\*").expect("Star regex should be valid");
    let mut sum1 = 0;
    let mut sum2 = 0;
    for line3 in input.lines().chain(iter::once("")) {
        let line3_syms = re_symbol.find_iter(line3).map(|m| m.start()).collect();
        let line3_nums = re_number
            .find_iter(line3)
            .map(|m| {
                (
                    m.start(),
                    m.end(),
                    m.as_str()
                        .parse()
                        .expect("Matched numbers should be integers"),
                )
            })
            .collect();
        'nums: for (start, end, num) in &line2_nums {
            for syms in [&line1_syms, &line2_syms, &line3_syms] {
                for sym in syms {
                    if start.saturating_sub(1) <= *sym && *sym < end + 1 {
                        sum1 += num;
                        continue 'nums;
                    }
                }
            }
        }
        'stars: for m in re_star.find_iter(line2) {
            let mut adjacent_count = 0;
            let mut gear_ratio = 1;
            for nums in [&line1_nums, &line2_nums, &line3_nums] {
                for (start, end, num) in nums {
                    if start.saturating_sub(1) <= m.start() && m.start() < end + 1 {
                        adjacent_count += 1;
                        gear_ratio *= num;
                        if adjacent_count > 2 {
                            // This isn't a gear
                            continue 'stars;
                        }
                    }
                }
            }
            if adjacent_count == 2 {
                sum2 += gear_ratio;
            }
        }
        line1_syms = line2_syms;
        line1_nums = line2_nums;
        line2 = line3;
        line2_syms = line3_syms;
        line2_nums = line3_nums;
    }
    println!("{sum1}");
    println!("{sum2}");
}

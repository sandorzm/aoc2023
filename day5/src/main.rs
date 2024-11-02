use regex::Regex;
use std::collections;
use std::fs;
use std::ops::Bound::{Excluded, Included, Unbounded};

// TODO Do this the other way. Compile mapping, then map each input number or range.
//      This is the only way to map numbers not in the mapping to themselves.

// Assume ranges of seeds don't overlap. Same for soils, fertilizers, etc.
fn main() {
    let input = fs::read_to_string("input1.txt").expect("Expected input.txt file");
    // Iterate over just the number parts of each mapping
    let re_map_boundary =
        Regex::new("\\n\\n+.* map:\\n").expect("Map boundary regex should be valid");
    let mut maps = re_map_boundary.split(&input);

    #[derive(Copy, Clone)]
    enum Endpoint {
        Start,
        End,
        Both,
    }
    let mut input_nums = collections::BTreeSet::new(); // Individual numbers for part 1
    let mut input_endpts = collections::BTreeMap::new(); // Inclusive range endpoints for part 2
    let mut last_start = None;
    // Special case: first "map" is seed list (skip first word "seeds:")
    for n in maps
        .next()
        .expect("Input should not be empty")
        .split_whitespace()
        .skip(1)
    {
        let n: u64 = n
            .parse()
            .expect("First line should have integer seed numbers");
        input_nums.insert(n);
        if let Some(start) = last_start {
            if n > 0 {
                input_endpts.insert(
                    start + (n - 1),
                    if n > 1 { Endpoint::End } else { Endpoint::Both },
                );
            }
            last_start = None;
        } else {
            input_endpts.insert(n, Endpoint::Start);
            last_start = Some(n);
        }
    }

    for map in maps {
        println!("{input_nums:?}");
        let mut output_nums = collections::BTreeSet::new();
        let mut output_endpts = collections::BTreeMap::new();
        for line in map.lines() {
            if let [dest, src, len] = line
                .split_whitespace()
                .map(|n| n.parse().expect("Map entries should be integers"))
                .collect::<Vec<u64>>()[..]
            {
                let range = (Included(src), Excluded(src + len));
                for n in input_nums.range(range) {
                    output_nums.insert(dest + (n - src));
                }
                // Handle if the mapping splits a range in input_endpts
                if let Some((_, Endpoint::Start)) =
                    input_endpts.range((Unbounded, Excluded(src))).rev().next()
                {
                    output_endpts.insert(dest, Endpoint::Start);
                }
                if let Some((_, Endpoint::End)) =
                    input_endpts.range((Included(src + len), Unbounded)).next()
                {
                    output_endpts.insert(dest + (len - 1), Endpoint::End);
                }
                for (&n, &t) in input_endpts.range(range) {
                    output_endpts.insert(dest + (n - src), t);
                }
            } else {
                panic!("Map lines should be three integers");
            }
        }
        input_nums = output_nums;
        input_endpts = output_endpts;
    }
    println!("{}", input_nums.first().unwrap_or(&0));
    println!("{}", input_endpts.first_key_value().map_or(0, |(k, _)| *k));
}

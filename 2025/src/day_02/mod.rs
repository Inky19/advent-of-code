use fancy_regex::Regex;
use std::{fs, ops::RangeInclusive};

pub fn run() {
    let file =
        fs::read_to_string("../data/2025/day_02/input.txt").expect("Could not read input.txt");
    let line = file.lines().next().unwrap(); // only one line in this problem
    pb1and2(line);
}

fn pb1and2(line: &str) {
    let reg_range = Regex::new("[0-9]+-[0-9]+").unwrap();
    let reg_num = Regex::new("[0-9]+").unwrap();

    let ranges: Vec<RangeInclusive<i64>> = reg_range
        .find_iter(line)
        .map(|range| {
            let bounds: Vec<i64> = reg_num
                .find_iter(range.unwrap().as_str())
                .map(|num| num.unwrap().as_str().parse::<i64>().unwrap())
                .collect();
            return RangeInclusive::new(bounds[0], bounds[1]);
        })
        .collect();

    let reg_invalid1 = Regex::new(r"\b(\d+)\1{1}\b").unwrap();
    let reg_invalid2 = Regex::new(r"\b(\d+)\1+\b").unwrap();
    let mut sum1 = 0;
    let mut sum2 = 0;
    for range in ranges {
        for i in range {
            if reg_invalid1.is_match(i.to_string().as_str()).unwrap() {
                sum1 += i;
                sum2 += i;
            } else if reg_invalid2.is_match(i.to_string().as_str()).unwrap() {
                sum2 += i;
            }
        }
    }
    println!("\n====== Problem 1 ======");
    println!("Sum of invalid IDs: {}", sum1);
    println!("\n====== Problem 2 ======");
    println!("Sum of invalid IDs: {}", sum2);
}

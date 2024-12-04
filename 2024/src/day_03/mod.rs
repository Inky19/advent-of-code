use regex::Regex;
use std::fs;

pub fn run() {
    let file =
        fs::read_to_string("../data/2024/day_03/input.txt").expect("Could not read input.txt");
    let lines: Vec<_> = file.lines().collect();
    pb1(&lines);
    pb2(&lines);
}

fn pb1(lines: &[&str]) {
    let reg_op = Regex::new(r"mul\([0-9]+,[0-9]+\)").unwrap();
    let reg_num = Regex::new("[0-9]+").unwrap();
    let sum = lines
        .iter()
        .map(|l| {
            reg_op
                .find_iter(l)
                .map(|mul| {
                    reg_num
                        .find_iter(mul.as_str())
                        .map(|m| m.as_str().parse::<i32>().unwrap())
                        .collect::<Vec<_>>()
                        .iter()
                        .product::<i32>()
                })
                .sum::<i32>()
        })
        .sum::<i32>();

    println!("\n====== Problem 1 ======");
    println!("Sum of all operations: {}", sum);
}

fn pb2(lines: &[&str]) {
    let reg_op = Regex::new(r"mul\([0-9]+,[0-9]+\)|do\(\)|don't\(\)").unwrap();
    let reg_num = Regex::new("[0-9]+").unwrap();
    let mut enable = true;
    let sum = lines
        .iter()
        .map(|l| {
            reg_op
                .find_iter(l)
                .filter_map(|exp| match exp.as_str() {
                    "do()" => {
                        enable = true;
                        None
                    }
                    "don't()" => {
                        enable = false;
                        None
                    }
                    _ => {
                        if !enable {
                            return None;
                        }
                        Some(
                            reg_num
                                .find_iter(exp.as_str())
                                .map(|m| m.as_str().parse::<i32>().unwrap())
                                .collect::<Vec<_>>()
                                .iter()
                                .product::<i32>(),
                        )
                    }
                })
                .sum::<i32>()
        })
        .sum::<i32>();

    println!("\n====== Problem 2 ======");
    println!("Sum of all operations: {}", sum);
}

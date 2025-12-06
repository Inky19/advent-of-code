use std::{
    fs,
    time::{SystemTime, UNIX_EPOCH},
    vec,
};

pub fn run() {
    let file =
        fs::read_to_string("../data/2025/day_06/input.txt").expect("Could not read input.txt");
    pb1(file.lines());
    let lines: Vec<_> = file.lines().collect();
    pb2(&lines);
}

fn pb1<'a>(lines: impl Iterator<Item = &'a str>) {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let mut lines = lines.peekable();
    let mut line = lines.next().unwrap();
    let mut numbers: Vec<Vec<u32>> = vec![];

    while lines.peek().is_some() {
        let mut line_numbers: Vec<u32> = vec![];
        for n in line.split_whitespace() {
            line_numbers.push(n.parse::<u32>().unwrap());
        }
        numbers.push(line_numbers);
        line = lines.next().unwrap();
    }

    let mut sum: u64 = 0;
    for (i, c) in line.split_whitespace().enumerate() {
        let mut line_sum = numbers[0][i];
        match c {
            "+" => numbers[1..].iter().for_each(|l| line_sum += l[i]),
            "*" => numbers[1..].iter().for_each(|l| line_sum *= l[i]),
            _ => {}
        }
        sum += line_sum as u64;
    }
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("\n====== Problem 1 ======");
    println!("Grand total: {}", sum);
    println!("Duration: {}s", (end - start).as_secs_f64());
}

fn pb2(lines: &[&str]) {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let line_size = lines[0].len();
    let mut lines_chars: Vec<Vec<char>> = vec![];
    lines
        .iter()
        .for_each(|l| lines_chars.push(l.chars().collect()));
    let max_digits = lines.len() - 1;
    let mut numbers: Vec<Vec<u32>> = vec![];
    let mut op_numbers: Vec<u32> = vec![];
    for col in 0..line_size {
        let mut empty_col = true;
        let mut number = 0;
        let mut power = 0;
        for line in (0..max_digits).rev() {
            let digit = lines_chars[line][col];
            if digit.is_digit(10) {
                empty_col = false;
                number += digit.to_digit(10).unwrap() * u32::pow(10, power);
                power += 1;
            }
        }

        if empty_col {
            numbers.push(op_numbers);
            op_numbers = vec![];
            continue;
        }
        op_numbers.push(number);
    }
    numbers.push(op_numbers);

    let last_line = lines.last().unwrap();
    let mut sum: u64 = 0;
    for (i, c) in last_line.split_whitespace().enumerate() {
        let mut line_sum = numbers[i][0] as u64;
        match c {
            "+" => numbers[i][1..].iter().for_each(|n| line_sum += *n as u64),
            "*" => numbers[i][1..].iter().for_each(|n| line_sum *= *n as u64),
            _ => {}
        }
        sum += line_sum as u64;
    }
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("\n====== Problem 2 ======");
    println!("Grand total: {}", sum);
    println!("Duration: {}s", (end - start).as_secs_f64());
}

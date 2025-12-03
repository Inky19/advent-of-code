use std::fs;

pub fn run() {
    let file =
        fs::read_to_string("../data/2025/day_03/input.txt").expect("Could not read input.txt");
    let lines: Vec<_> = file.lines().collect();
    pb1(&lines);
    pb2(&lines);
}

fn pb1(lines: &[&str]) {
    let mut sum = 0;

    for line in lines {
        let mut max_d = 0;
        let mut max_u = 0;
        let mut it = line.chars().peekable();
        while let Some(c) = it.next() {
            let digit = c.to_digit(10).unwrap();
            if !it.peek().is_none() && digit > max_d {
                max_d = digit;
                max_u = 0;
            } else if digit > max_u {
                max_u = digit;
            }
        }
        sum += max_d * 10 + max_u;
    }
    println!("\n====== Problem 1 ======");
    println!("Max joltage: {}", sum);
}

fn pb2(lines: &[&str]) {
    let mut sum: u64 = 0;

    for line in lines {
        let mut cur_max: u64 = 0;
        let line_string = String::from(*line);
        let mut slice = line_string.as_str();
        let mut pos = 0;
        for i in 0..12 {
            let (digit, rel_pos) = get_first_digit_and_pos(slice, 12 - i);
            pos += rel_pos + 1;
            slice = &line_string[pos as usize..];
            cur_max += digit as u64 * u64::pow(10, 11 - i);
        }
        sum += cur_max;
    }
    println!("\n====== Problem 2 ======");
    println!("Max joltage: {}", sum);
}

fn get_first_digit_and_pos(digits: &str, size: u32) -> (u32, u32) {
    let mut max = 0;
    let mut pos = 0;
    let mut it = digits.chars().peekable();
    for i in 0..(digits.len() as u32 - size + 1) {
        let digit = it.next().unwrap().to_digit(10).unwrap();
        if digit > max {
            max = digit;
            pos = i;
        }
    }
    return (max, pos);
}

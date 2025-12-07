use std::{
    fs,
    time::{SystemTime, UNIX_EPOCH},
};

pub fn run() {
    let file =
        fs::read_to_string("../data/2025/day_07/input.txt").expect("Could not read input.txt");
    pb1(file.lines());
    pb2(file.lines());
}

fn pb1<'a>(mut lines: impl Iterator<Item = &'a str>) {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let line = lines.next().unwrap();
    let width = line.len();
    let mut tachyon_line: Vec<bool> = vec![false; width + 2];
    for c in line.chars().enumerate() {
        if c.1 == 'S' {
            tachyon_line[c.0 + 1] = true;
            break;
        }
    }
    let mut sum = 0;

    for line in lines {
        for c in line.chars().enumerate() {
            if tachyon_line[c.0 + 1] && c.1 == '^' {
                sum += 1;
                tachyon_line[c.0] = true;
                tachyon_line[c.0 + 1] = false;
                tachyon_line[c.0 + 2] = true;
            }
        }
    }
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("\n====== Problem 1 ======");
    println!("Number of time the beam is split: {}", sum);
    println!("Duration: {}s", (end - start).as_secs_f64());
}

fn pb2<'a>(mut lines: impl Iterator<Item = &'a str>) {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let line = lines.next().unwrap();
    let width = line.len();
    let mut tachyon_line: Vec<u64> = vec![0; width + 2];
    for c in line.chars().enumerate() {
        if c.1 == 'S' {
            tachyon_line[c.0 + 1] = 1;
            break;
        }
    }

    for line in lines {
        for c in line.chars().enumerate() {
            let beam_counter = tachyon_line[c.0 + 1];
            if beam_counter > 0 && c.1 == '^' {
                tachyon_line[c.0] += beam_counter;
                tachyon_line[c.0 + 1] = 0;
                tachyon_line[c.0 + 2] += beam_counter;
            }
        }
    }
    let sum: u64 = tachyon_line.iter().sum();
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("\n====== Problem 2 ======");
    println!("Number of timelines: {}", sum);
    println!("Duration: {}s", (end - start).as_secs_f64());
}

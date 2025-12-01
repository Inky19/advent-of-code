use std::fs;

pub fn run() {
    let file =
        fs::read_to_string("../data/2025/day_01/input.txt").expect("Could not read input.txt");
    let lines: Vec<_> = file.lines().collect();
    pb1and2(&lines);
}

fn pb1and2(lines: &[&str]) {
    let mut dial = 50;
    let mut counter1 = 0;
    let mut counter2 = 0;
    for line in lines {
        let steps = line[1..].parse::<i32>().unwrap();
        let delta = if line.starts_with("L") { -steps } else { steps };
        let zstart = dial == 0;

        dial += delta;
        counter2 += (dial / 100).abs();
        dial %= 100;

        if dial < 0 {
            dial = 100 + dial;
            if !zstart {
                // Possible non-complete rotation over 0
                counter2 += 1;
            }
        }

        if dial == 0 {
            counter1 += 1;
            if delta < 0 {
                // Positive case handled by counter2 += dial / 100
                counter2 += 1;
            }
        }
    }
    println!("\n====== Problem 1 ======");
    println!("Password: {}", counter1);
    println!("\n====== Problem 2 ======");
    println!("Password: {}", counter2);
}

use regex::Regex;
use std::fs;

pub fn run() {
    let file =
        fs::read_to_string("../data/2024/day_02/input.txt").expect("Could not read input.txt");
    let lines: Vec<_> = file.lines().collect();
    pb1(&lines);
    pb2(&lines);
}

fn pb1(lines: &[&str]) {
    let reg = Regex::new("[0-9]+").unwrap();
    let mut unsafe_sum = 0;
    for l in lines {
        let extract: Vec<&str> = reg.find_iter(l).map(|m| m.as_str()).collect();
        if extract.len() <= 1 {
            continue;
        }
        let val0 = extract[0].parse::<i32>().unwrap();
        let val1 = extract[1].parse::<i32>().unwrap();
        if !check_range(val0, val1) {
            unsafe_sum += 1;
            continue;
        }
        let increasing = val0 < val1;
        let mut prev = val1;
        for s in &extract[2..] {
            let cur = s.parse::<i32>().unwrap();

            if !valid_elements(cur, prev, increasing) {
                unsafe_sum += 1;
                break;
            }
            prev = cur;
        }
    }
    println!("\n====== Problem 1 ======");
    let num_safe_reports = lines.len() - unsafe_sum;
    println!("Number of safe reports: {}", num_safe_reports);
}

fn check_range(cur: i32, prev: i32) -> bool {
    let abs_diff = i32::abs(cur - prev);
    abs_diff <= 3 && abs_diff > 0
}

fn valid_elements(cur: i32, prev: i32, increasing: bool) -> bool {
    check_range(cur, prev) && !(increasing && cur < prev) && !(!increasing && cur > prev)
}

fn pb2(lines: &[&str]) {
    let reg = Regex::new("[0-9]+").unwrap();
    let mut safe_report_sum = 0;
    for l in lines {
        let extract: Vec<&str> = reg.find_iter(l).map(|m| m.as_str()).collect();
        if extract.len() <= 2 {
            continue;
        }

        let init_params = [(None, 0, 1), (Some(0), 1, 2), (Some(1), 0, 2)];
        //let init_params = [(None, 0, 1)];

        let mut safe_report = false;

        for params in &init_params {
            if safe_report {
                break;
            }
            let mut buffer_index = params.0;
            let mut val0 = extract[params.1].parse::<i32>().unwrap();
            let mut val1 = extract[params.2].parse::<i32>().unwrap();
            if !check_range(val0, val1) {
                continue;
            }

            let increasing = val0 < val1;

            let mut scenario_unsafe = false;
            // Use a sliding window of 3 elements to check if the sequence is safe ([val0, val1, cur])
            for i in params.2 + 1..extract.len() {
                let cur = extract[i].parse::<i32>().unwrap();
                match buffer_index {
                    None => {
                        if !valid_elements(cur, val1, increasing) {
                            buffer_index = Some(i);
                        }
                    }
                    Some(index) if index == i - 1 => {
                        if !valid_elements(cur, val0, increasing) {
                            if i < 3 {
                                scenario_unsafe = true;
                                break;
                            }
                            // Rewind and check if removing the first element and keeping the last two is safe
                            let old_val0 = extract[i - 3].parse::<i32>().unwrap();
                            if (!valid_elements(val1, old_val0, increasing))
                                || (!valid_elements(cur, val1, increasing))
                            {
                                scenario_unsafe = true;
                                break;
                            } else {
                                buffer_index = Some(i - 2);
                            }
                        }
                    }
                    Some(_) => {
                        if !valid_elements(cur, val1, increasing) {
                            scenario_unsafe = true;
                            break;
                        }
                    }
                }
                val0 = val1;
                val1 = cur;
            }
            if !scenario_unsafe {
                safe_report = true;
            }
        }
        if safe_report {
            safe_report_sum += 1;
        }
    }
    println!("\n====== Problem 2 ======");
    println!("Number of safe reports: {}", safe_report_sum);
}

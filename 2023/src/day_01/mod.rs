use std::fs;
use std::collections::HashMap;



pub fn run(){
    let file = fs::read_to_string("../data/2023/day_01/input.txt")
    .expect("Could not read input.txt");
    let lines:Vec<_> = file.lines().collect();
    println!("\n====== Problem 1 ======");
    pb1(&lines);
    println!("\n====== Problem 2 ======");
    pb2(&lines);
}


fn pb1(lines:&[&str]){
    let mut val = 0;
    for line in lines {
        let mut d_first:Option<u32> = None;
        let mut d_last:Option<u32> = None;
        for c in line.chars(){
            if c.is_numeric() {
                if d_first.is_none() {
                    d_first = c.to_digit(10);
                }
                d_last = c.to_digit(10);
                
            }
        }
        
        match d_first {
            None => println!("First digit of line {} not found", line),
            Some(d_f) => val += 10*d_f
        }
        match d_last {
            None => println!("Last digit of line {} not found", line),
            Some(d_l) => val += d_l
        }
    }
    println!("[pb1] Sum of all lines: {}",val);
}

fn pb2(lines:&[&str]){
    // Build hashmap
    const DIGITS_STR:[&str; 10] = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut num_hash:HashMap<String, usize> = HashMap::new();
    for i in 0..DIGITS_STR.len() {
        num_hash.insert(i.to_string(), i);
        num_hash.insert(DIGITS_STR[i].into(), i);
    }

    // Search each substring in order for each line
    let mut val = 0;
    for line in lines {
        let mut d_first:Option<usize> = None;
        let mut d_last:Option<usize> = None;
        for c in 0..line.len(){
            let mut subline:String = String::new();
            for i in c..line.len(){
                subline += line.chars().nth(i).unwrap().to_string().as_str();
                let mut next_char = true;
                for (k, _) in &num_hash {
                    if k.starts_with(&subline) {
                        next_char = false;
                    }
                }
                if next_char {break;}
                if num_hash.contains_key(&subline){
                    if d_first.is_none() {
                        d_first = Some(*num_hash.get(&subline).unwrap());
                    }
                    d_last = Some(*num_hash.get(&subline).unwrap());
                }

            }
        }
        match d_first {
            None => println!("First digit of line {} not found", line),
            Some(d_f) => val += 10*d_f
        }
        match d_last {
            None => println!("Last digit of line {} not found", line),
            Some(d_l) => val += d_l
        }
    }
    println!("[pb2] Sum of all lines: {}",val);
}
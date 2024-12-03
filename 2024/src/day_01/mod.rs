use regex::Regex;
use std::{collections::HashMap, fs};

pub fn run() {
    let file =
        fs::read_to_string("../data/2024/day_01/input.txt").expect("Could not read input.txt");
    let lines: Vec<_> = file.lines().collect();
    pb1and2(&lines);
}

fn pb1and2(lines: &[&str]) {
    let reg = Regex::new("[0-9]+").unwrap();
    let mut list_l = Vec::new();
    let mut list_r = Vec::new();

    let mut hash_map_l: HashMap<i32, i32> = HashMap::new();
    let mut hash_map_r: HashMap<i32, i32> = HashMap::new();
    for l in lines {
        let extract: Vec<&str> = reg.find_iter(l).map(|m| m.as_str()).collect();
        let val_l = extract[0].parse::<i32>().unwrap();
        let val_r = extract[1].parse::<i32>().unwrap();
        list_l.push(val_l);
        list_r.push(val_r);
        hash_map_count(&mut hash_map_l, val_l);
        hash_map_count(&mut hash_map_r, val_r);
    }
    assert!(list_l.len() == list_r.len(), "List lengths do not match");
    list_l.sort();
    list_r.sort();
    let mut distances_sum = 0;

    for i in 0..list_l.len() {
        distances_sum += i32::abs(list_r[i] - list_l[i]);
    }

    let mut similarities_sum = 0;
    for key in hash_map_l.keys() {
        if hash_map_r.contains_key(key) {
            similarities_sum += key * hash_map_l[key] * hash_map_r[key];
        }
    }
    println!("\n====== Problem 1 ======");
    println!("[pb1] Sum of all distances: {}", distances_sum);
    println!("\n====== Problem 2 ======");
    println!("[pb1] Sum of all similarities: {}", similarities_sum);
}

fn hash_map_count(hash_map: &mut HashMap<i32, i32>, val: i32) {
    if hash_map.contains_key(&val) {
        hash_map.insert(val, hash_map[&val] + 1);
    } else {
        hash_map.insert(val, 1);
    }
}

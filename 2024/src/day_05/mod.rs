use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

pub fn run() {
    let file =
        fs::read_to_string("../data/2024/day_05/input.txt").expect("Could not read input.txt");
    let lines: Vec<_> = file.lines().collect();
    pb1and2(&lines);
}

fn pb1and2(lines: &[&str]) {
    let order_reg = Regex::new(r"[0-9]+\|[0-9]+").unwrap();
    let mut orders: HashMap<i32, Vec<i32>> = HashMap::new();

    let mut index = 0;
    while lines[index] != "" && index < lines.len() {
        let caps = order_reg.captures(lines[index]);
        if caps.is_none() {
            println!("Invalid line: {:?}", lines[index]);
            break;
        }
        let order: Vec<i32> = caps.unwrap()[0]
            .split('|')
            .map(|x| x.parse().unwrap())
            .collect();
        orders
            .entry(order[0])
            .or_insert_with(Vec::new)
            .push(order[1]);
        index += 1;
    }
    index += 1;

    let num_reg = Regex::new(r"[0-9]+").unwrap();
    let mut sum = 0;
    let mut invalid_pages: Vec<Vec<i32>> = Vec::new();
    for l in &lines[index..lines.len()] {
        let mut pages: Vec<i32> = num_reg
            .find_iter(l)
            .map(|m| m.as_str().parse::<i32>().unwrap())
            .collect();
        let mut constraints: HashSet<i32> = HashSet::new();

        let mut valid = true;

        pages.reverse();

        for p in &pages {
            if constraints.contains(p) {
                valid = false;
                break;
            }

            let prev_c = orders.get(&p);
            if prev_c.is_none() {
                continue;
            }
            let prev_c = prev_c.unwrap();
            for c in prev_c {
                constraints.insert(*c);
            }
        }

        if valid {
            sum += pages[pages.len() / 2];
        } else {
            invalid_pages.push(pages);
        }
    }
    println!("\n====== Problem 1 ======");
    println!("Sum of middle page number for valid updates: {:?}", sum);

    // Brute force but it works ^w^'
    sum = 0;
    for mut pages in invalid_pages {
        let mut invalid = true;
        while invalid {
            invalid = false;
            for i in 0..pages.len() {
                let constraints = orders.get(&pages[i]);
                if constraints.is_none() {
                    continue;
                }
                let constraints = constraints.unwrap();
                for j in i + 1..pages.len() {
                    if constraints.contains(&pages[j]) {
                        let tmp = pages[i];
                        pages[i] = pages[j];
                        pages[j] = tmp;

                        invalid = true;
                        break;
                    }
                }

                if invalid {
                    break;
                }
            }
        }
        sum += pages[pages.len() / 2];
    }
    println!("\n====== Problem 2 ======");
    println!(
        "Sum of middle page number for corrected invalid updates: {:?}",
        sum
    );
}

use itertools::Itertools;
use std::{
    fs,
    time::{SystemTime, UNIX_EPOCH},
};

pub fn run() {
    let file =
        fs::read_to_string("../data/2025/day_09/input.txt").expect("Could not read input.txt");
    pb1(file.lines());
}
#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

fn pb1<'a>(lines: impl Iterator<Item = &'a str>) {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let points: Vec<Point> = lines
        .map(|l| {
            let coords: Vec<i32> = l
                .split(',')
                .map(|coord| coord.parse::<i32>().unwrap())
                .collect();
            return Point {
                x: coords[0],
                y: coords[1],
            };
        })
        .collect();

    let areas: Vec<u64> = points
        .iter()
        .cloned()
        .combinations(2)
        .map(|v| {
            let width = v[0].x - v[1].x;
            let height = v[0].y - v[1].y;
            return (width.abs() + 1) as u64 * (height.abs() + 1) as u64;
        })
        .collect();
    let max_area = areas.iter().max().unwrap();
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("\n====== Problem 1 ======");
    println!("Max area: {}", max_area);
    println!("Duration: {}s", (end - start).as_secs_f64());
}

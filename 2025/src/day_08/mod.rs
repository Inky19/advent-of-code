use itertools::Itertools;
use std::{
    collections::HashSet,
    fs,
    time::{SystemTime, UNIX_EPOCH},
};

pub fn run() {
    let file =
        fs::read_to_string("../data/2025/day_08/input.txt").expect("Could not read input.txt");
    pb1and2(file.lines());
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

struct Link {
    p1: Point,
    p2: Point,
    distance: u64,
}

impl Point {
    fn compute_squared_distance(&self, o: &Point) -> u64 {
        let dx = self.x - o.x;
        let dy = self.y - o.y;
        let dz = self.z - o.z;
        return u64::pow(dx as u64, 2) + u64::pow(dy as u64, 2) + u64::pow(dz as u64, 2);
    }
}

fn pb1and2<'a>(lines: impl Iterator<Item = &'a str>) {
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
                z: coords[2],
            };
        })
        .collect();

    let mut distances: Vec<Link> = points
        .iter()
        .cloned()
        .combinations(2)
        .map(|v| Link {
            p1: v[0],
            p2: v[1],
            distance: v[0].compute_squared_distance(&v[1]),
        })
        .collect();

    distances.sort_by(|a, b| a.distance.cmp(&b.distance));

    let max_links = 1000;
    let mut circuits: Vec<HashSet<Point>> = vec![];
    let mut link_sum = 0;
    let mut distances_iter = distances.iter();

    while link_sum < max_links
        && let Some(distance) = distances_iter.next()
    {
        build_circuits(&mut circuits, distance);
        link_sum += 1;
    }
    circuits.sort_by(|a, b| b.len().cmp(&a.len()));
    let mut sum = 1;
    for i in 0..3 {
        sum *= circuits[i].len();
    }
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("\n====== Problem 1 ======");
    println!("Value of the 3 biggest circuits: {}", sum);
    println!("Duration: {}s", (end - start).as_secs_f64());

    let mut last_distance: Option<&Link> = None;
    let mut complete_graph = false;
    while !complete_graph && let Some(distance) = distances_iter.next() {
        build_circuits(&mut circuits, distance);
        last_distance = Some(distance);
        complete_graph = circuits.len() == 1 && circuits[0].len() == points.len();
    }
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("\n====== Problem 2 ======");
    if let Some(last_distance) = last_distance {
        let sum2 = last_distance.p1.x as u64 * last_distance.p2.x as u64;
        println!("Last connection: {}", sum2);
        println!("Duration: {}s", (end - start).as_secs_f64());
    } else {
        println!("\nERROR: Could not generate complete graph!");
    }
}

fn build_circuits(circuits: &mut Vec<HashSet<Point>>, distance: &Link) {
    let target_junction = distance.p2;
    let junction = distance.p1;
    let target_circuit_id = circuits
        .iter_mut()
        .position(|circuit| circuit.contains(&target_junction));
    let circuit_id = circuits
        .iter_mut()
        .position(|circuit| circuit.contains(&junction));

    if let Some(target_circuit_id) = target_circuit_id {
        if let Some(circuit_id) = circuit_id {
            if circuit_id == target_circuit_id {
                return;
            }
            let (circuit, target_circuit) = if circuit_id < target_circuit_id {
                let (left, right) = circuits.split_at_mut(target_circuit_id);
                (&mut left[circuit_id], &mut right[0])
            } else {
                let (left, right) = circuits.split_at_mut(circuit_id);
                (&mut right[0], &mut left[target_circuit_id])
            };
            circuit.extend(target_circuit.drain());
            circuits.remove(target_circuit_id);
        } else {
            circuits[target_circuit_id].insert(junction.clone());
        }
    } else {
        if let Some(circuit_id) = circuit_id {
            circuits[circuit_id].insert(target_junction.clone());
        } else {
            let mut new_circuit = HashSet::new();
            new_circuit.insert(junction.clone());
            new_circuit.insert(target_junction.clone());
            circuits.push(new_circuit);
        }
    }
}

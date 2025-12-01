use std::{collections::HashSet, fs};

pub fn run() {
    let file =
        fs::read_to_string("../data/2024/day_06/input.txt").expect("Could not read input.txt");
    let lines: Vec<_> = file.lines().collect();
    pb1and2(&lines);
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Guard {
    pos: (i32, i32),
    dir: Direction,
    visited: HashSet<(i32, i32)>,
}

impl Guard {
    fn new_pos(&self) -> (i32, i32) {
        match self.dir {
            Direction::Up => (self.pos.0, self.pos.1 - 1),
            Direction::Down => (self.pos.0, self.pos.1 + 1),
            Direction::Left => (self.pos.0 - 1, self.pos.1),
            Direction::Right => (self.pos.0 + 1, self.pos.1),
        }
    }
}

fn pb1and2(lines: &Vec<&str>) {
    let mut guard: Guard = Guard {
        pos: (-1, -1),
        dir: Direction::Up,
        visited: HashSet::new(),
    };

    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '^' {
                guard.pos = (j as i32, i as i32);
                break;
            }
            if guard.pos.0 != -1 {
                break;
            }
        }
    }

    let init_pos = guard.pos;

    if guard.pos.0 == -1 {
        println!("Error: Guard not found");
        return;
    } else {
        println!("Guard found at ({}, {})", guard.pos.0, guard.pos.1);
    }

    let width = lines[0].len() as i32;
    let height = lines.len() as i32;

    guard.visited.insert(guard.pos);

    while is_inside(guard.pos, width, height) {
        let next_pos = guard.new_pos();
        if !is_inside(next_pos, width, height) {
            break;
        }
        if colliding_map(&mut guard, lines, next_pos) {
            continue;
        }
        guard.visited.insert(next_pos);
        guard.pos = next_pos;
    }

    println!("\n====== Problem 1 ======");
    println!("Guard made {} steps", guard.visited.len());

    //problem 2

    let mut sum = 0;
    for v in guard.visited.clone().iter() {
        guard.pos = init_pos;
        guard.dir = Direction::Up;
        let mut encountered_obstructions: HashSet<(i32, i32, Direction)> = HashSet::new();
        while is_inside(guard.pos, width, height) {
            let new_pos = guard.new_pos();
            if !is_inside(new_pos, width, height) {
                break;
            }
            if colliding_map_or_obstruction(&mut guard, lines, new_pos, Some(v.clone())) {
                if !encountered_obstructions.insert((guard.pos.0, guard.pos.1, guard.dir)) {
                    sum += 1;
                    break;
                }
                continue;
            }
            guard.visited.insert(new_pos);
            guard.pos = new_pos;
        }
    }
    println!("\n====== Problem 2 ======");
    println!("Possible new obstructions for loop: {}", sum);
}

fn is_inside(pos: (i32, i32), width: i32, height: i32) -> bool {
    pos.0 >= 0 && pos.0 < width as i32 && pos.1 >= 0 && pos.1 < height as i32
}

fn colliding_map(guard: &mut Guard, lines: &Vec<&str>, new_pos: (i32, i32)) -> bool {
    colliding_map_or_obstruction(guard, lines, new_pos, None)
}

fn colliding_map_or_obstruction(
    guard: &mut Guard,
    lines: &Vec<&str>,
    new_pos: (i32, i32),
    new_obstruction: Option<(i32, i32)>,
) -> bool {
    if lines[new_pos.1 as usize]
        .chars()
        .nth(new_pos.0 as usize)
        .unwrap()
        == '#'
        || new_obstruction.is_some_and(|obs_pos| obs_pos == new_pos)
    {
        match guard.dir {
            Direction::Up => guard.dir = Direction::Right,
            Direction::Down => guard.dir = Direction::Left,
            Direction::Left => guard.dir = Direction::Up,
            Direction::Right => guard.dir = Direction::Down,
        }
        return true;
    }
    return false;
}

use std::{
    fs,
    time::{SystemTime, UNIX_EPOCH},
};

pub fn run() {
    let file =
        fs::read_to_string("../data/2025/day_04/input.txt").expect("Could not read input.txt");
    let lines = file.lines();
    pb1(lines);
    pb2(file.lines());
}

struct GridSlice {
    width: usize,
    lines: [Vec<bool>; 3],
    buffer_offset: usize,
}

impl GridSlice {
    pub fn new(width: usize) -> Self {
        GridSlice {
            width,
            lines: [
                vec![false; width + 2],
                vec![false; width + 2],
                vec![false; width + 2],
            ],
            buffer_offset: 0,
        }
    }

    pub fn load_line_str(&mut self, line: &str) {
        self.buffer_offset = (self.buffer_offset + 1) % 3;
        for c in line.chars().enumerate() {
            self.lines[self.buffer_offset][c.0 + 1] = c.1 == '@';
        }
    }

    pub fn load_line_vec(&mut self, line: &Vec<bool>) {
        self.buffer_offset = (self.buffer_offset + 1) % 3;
        self.lines[self.buffer_offset] = vec![false];
        self.lines[self.buffer_offset].append(&mut line.clone());
        self.lines[self.buffer_offset].push(false);
    }

    pub fn compute_line_pb2(&mut self, line: &mut Vec<bool>) -> u32 {
        let mut sum = 0;
        let prev_line = &self.lines[(self.buffer_offset + 1) % 3];
        let cur_line = &self.lines[(self.buffer_offset + 2) % 3];
        let next_line = &self.lines[self.buffer_offset];
        for b in cur_line.iter().enumerate() {
            if *b.1 {
                let mut neighbours = 0;
                for i in 0..3 {
                    neighbours += prev_line[b.0 + 1 - i] as u32;
                    neighbours += next_line[b.0 + 1 - i] as u32;
                }
                neighbours += cur_line[b.0 - 1] as u32;
                neighbours += cur_line[b.0 + 1] as u32;

                if neighbours < 4 {
                    sum += (neighbours < 4) as u32;
                    line[b.0 - 1] = false;
                }
            }
        }
        return sum;
    }

    pub fn add_empty_line(&mut self) {
        self.buffer_offset = (self.buffer_offset + 1) % 3;
        self.lines[self.buffer_offset] = vec![false; self.width + 2]
    }

    pub fn count_rolls_current_line(&self) -> u32 {
        let mut sum = 0;
        let prev_line = &self.lines[(self.buffer_offset + 1) % 3];
        let line = &self.lines[(self.buffer_offset + 2) % 3];
        let next_line = &self.lines[self.buffer_offset];
        for b in line.iter().enumerate() {
            if *b.1 {
                let mut neighbours = 0;
                for i in 0..3 {
                    neighbours += prev_line[b.0 + 1 - i] as u32;
                    neighbours += next_line[b.0 + 1 - i] as u32;
                }
                neighbours += line[b.0 - 1] as u32;
                neighbours += line[b.0 + 1] as u32;

                sum += (neighbours < 4) as u32;
            }
        }
        return sum;
    }
}

fn pb1<'a>(mut lines: impl Iterator<Item = &'a str>) {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    let mut sum = 0;
    let first_line = lines.next().unwrap();
    let mut grid = GridSlice::new(first_line.len());
    grid.add_empty_line();
    grid.load_line_str(first_line);
    while let Some(line) = lines.next() {
        grid.load_line_str(line);
        sum += grid.count_rolls_current_line();
    }
    grid.add_empty_line();
    sum += grid.count_rolls_current_line();

    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    println!("\n====== Problem 1 ======");
    println!("Number of accessible rolls {}", sum);
    println!("Duration: {}s", (end - start).as_secs_f64());
}

fn pb2<'a>(lines: impl Iterator<Item = &'a str>) {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let mut grid: Vec<Vec<bool>> = vec![];
    for line in lines {
        grid.push(vec![]);
        for c in line.chars() {
            grid.last_mut().unwrap().push(c == '@');
        }
    }
    let width = grid.first().unwrap().len();
    let mut grid_slice = GridSlice::new(width);

    let mut sum = 0;
    let mut step_sum = u32::MAX;
    while step_sum != 0 {
        step_sum = 0;
        let mut grid_it = grid.iter_mut();
        let mut line = grid_it.next().unwrap();
        grid_slice.add_empty_line();
        grid_slice.load_line_vec(line);
        for next_line in grid_it {
            grid_slice.load_line_vec(&next_line);
            step_sum += grid_slice.compute_line_pb2(&mut line);
            line = next_line;
        }
        grid_slice.add_empty_line();
        step_sum += grid_slice.compute_line_pb2(line);
        sum += step_sum;
    }

    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    println!("\n====== Problem 2 ======");
    println!("Number of removed rolls {}", sum);
    println!("Duration: {}s", (end - start).as_secs_f64());
}

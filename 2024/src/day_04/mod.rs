use std::{collections::HashMap, fs};

pub fn run() {
    let file =
        fs::read_to_string("../data/2024/day_04/input.txt").expect("Could not read input.txt");
    let lines: Vec<_> = file.lines().collect();
    pb1(&lines);
    pb2(&lines);
}

fn pb1(lines: &[&str]) {
    let width = lines[0].len();
    let height = lines.len();
    let word_len = 4;
    let text: Vec<char> = lines.concat().chars().collect();

    let mut sum = 0;

    for i in 0..text.len() {
        if text[i] != 'X' {
            continue;
        }

        let x = i % width;
        let y = i / width;

        // Horizontal
        if x < width - word_len + 1
            && text[i + 1] == 'M'
            && text[i + 2] == 'A'
            && text[i + 3] == 'S'
        {
            sum += 1;
        }
        if x >= word_len - 1 && text[i - 1] == 'M' && text[i - 2] == 'A' && text[i - 3] == 'S' {
            sum += 1;
        }

        // Vertical
        if y < height - word_len + 1
            && text[i + width] == 'M'
            && text[i + 2 * width] == 'A'
            && text[i + 3 * width] == 'S'
        {
            sum += 1;
        }

        if y >= word_len - 1
            && text[i - width] == 'M'
            && text[i - 2 * width] == 'A'
            && text[i - 3 * width] == 'S'
        {
            sum += 1;
        }

        // Diagonal
        // Top left to bottom right
        if x < width - word_len + 1 && y < height - word_len + 1 {
            if text[i + width + 1] == 'M'
                && text[i + 2 * width + 2] == 'A'
                && text[i + 3 * width + 3] == 'S'
            {
                sum += 1;
            }
        }

        if x >= word_len - 1 && y >= word_len - 1 {
            if text[i - width - 1] == 'M'
                && text[i - 2 * width - 2] == 'A'
                && text[i - 3 * width - 3] == 'S'
            {
                sum += 1;
            }
        }

        // Top right to bottom left
        if x < width - word_len + 1 && y >= word_len - 1 {
            if text[i - width + 1] == 'M'
                && text[i - 2 * width + 2] == 'A'
                && text[i - 3 * width + 3] == 'S'
            {
                sum += 1;
            }
        }

        if x >= word_len - 1 && y < height - word_len + 1 {
            if text[i + width - 1] == 'M'
                && text[i + 2 * width - 2] == 'A'
                && text[i + 3 * width - 3] == 'S'
            {
                sum += 1;
            }
        }
    }

    println!("\n====== Problem 1 ======");
    println!("Number of XMAS: {}", sum);
}

fn pb2(lines: &[&str]) {
    let width = lines[0].len();
    let height = lines.len();
    let text: Vec<char> = lines.concat().chars().collect();

    let mut sum = 0;
    let hash: HashMap<char, char> = [('M', 'S'), ('S', 'M')].iter().cloned().collect();

    for i in 0..text.len() {
        let x = i % width;
        let y = i / width;

        if text[i] != 'A' || x == 0 || y == 0 || x >= width - 1 || y >= height - 1 {
            continue;
        }

        let top_left_char = text[i - width - 1];
        let bottom_left_char = text[i + width - 1];

        let top_right_char = text[i - width + 1];
        let bottom_right_char = text[i + width + 1];

        let diags = vec![
            [top_left_char, bottom_right_char],
            [top_right_char, bottom_left_char],
        ];

        let mut invalid = false;
        for d in diags {
            if invalid || hash.get(&d[0]) != Some(&d[1]) {
                invalid = true;
                continue;
            }
        }
        if invalid {
            continue;
        }

        sum += 1;
    }

    println!("\n====== Problem 2 ======");
    println!("Number of XMAS: {}", sum);
}

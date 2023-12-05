use std::fs;

pub fn run(){
    let file = fs::read_to_string("../data/2023/day_03/input.txt")
    .expect("Could not read input.txt");
    let lines:Vec<_> = file.lines().collect();
    println!("\n====== Problem 1 ======");
    pb1(&lines);
    println!("\n====== Problem 2 ======");
    pb2(&lines);
}

fn pb1(lines:&[&str]){
    let line_len = lines[0].len();
    let mut number:u32 = 0;
    let mut number_i:u32 = 1;
    let mut sum = 0;
    for l in 0..lines.len() {
        let mut valid = false;
        for c in (0..line_len).rev() {
            let ch = lines[l].chars().nth(c).unwrap();
            if ch.is_numeric() {
                number+=ch.to_digit(10).unwrap()*number_i;
                number_i = number_i*10;
                if check_if_symbol_around(&lines, l as u32, c as u32) {
                    valid = true;
                }
            } 
            if !ch.is_numeric() || c==0 {
                if valid {
                    sum += number;
                }
                number = 0;
                number_i = 1;
                valid = false;
            }
        }
    }
    println!("{}", sum);
}

fn check_if_symbol_around(lines:&[&str], row:u32, col:u32) -> bool{
    for i in -1..=1 {
        let row_i = row as i32+i;
        if row_i < 0 || row_i >= lines.len() as i32 {continue;}
        for j in -1..=1 {
            let col_j = col as i32+j;
            if col_j < 0 || col_j >= lines[0].len() as i32 {break;}
            if i == 0 && j == 0 {continue;}
            let ch = lines[row_i as usize].chars().nth(col_j as usize).unwrap();
            if  ch != '.' && !ch.is_numeric(){
                return true; 
            }
        }
    }
    return false;
}

fn pb2(lines:&[&str]){
    let line_len = lines[0].len();
    let mut sum = 0;
    for l in 0..lines.len() {
        for c in (0..line_len).rev() {
            let ch = lines[l].chars().nth(c).unwrap();
            if ch == '*' {
                let ratio = get_gear_ratio(&lines, l as u32, c as u32);
                if ratio.is_valid{
                    sum += ratio.ratio[0] * ratio.ratio[1];
                }
            }
        }
    }
    println!("{}", sum);
}

struct GearRatio{
    is_valid:bool,
    ratio:[u32;2]
}

fn get_gear_ratio(lines:&[&str], row:u32, col:u32) -> GearRatio{
    let mut ratio = GearRatio{
        is_valid: false,
        ratio: [0,0]
    };
    let mut num_of_num = 0;
    let mut coords_num = [[0,0],[0,0]];

    for i in -1..=1 {
        let row_i = row as i32+i;
        if row_i < 0 || row_i >= lines.len() as i32 {continue;}
        let j_range = [0, -1, 1];
        for j in  j_range{
            let col_j = col as i32+j;
            if col_j < 0 || col_j >= lines[0].len() as i32 {break;}
            if i == 0 && j == 0 {continue;}
            let ch = lines[row_i as usize].chars().nth(col_j as usize).unwrap();
            if ch.is_digit(10){
                coords_num[num_of_num][0] = row_i as u32;
                coords_num[num_of_num][1] = col_j as u32;
                num_of_num +=1;
                if num_of_num > 2 {return ratio;}
                
                if j==0 {break;}
            }
        }
    }
    if num_of_num != 2 {return ratio;}
    ratio.is_valid = true;
    for i in 0..2 {
        let mut number = 0;
        let mut power = 1;
        let mut start_col = coords_num[i][1] as i32;
        while start_col<(lines[0].len()-1).try_into().unwrap() {
            start_col += 1;
            if !lines[coords_num[i][0] as usize].chars().nth(start_col as usize).unwrap().is_digit(10) {
                start_col -=1;
                break;
            }
        }
        while start_col>=0 {
            let digit = lines[coords_num[i][0] as usize].chars().nth(start_col.try_into().unwrap()).unwrap();
            if !digit.is_digit(10) {break;}
            number += digit.to_digit(10).unwrap()*power;
            power = power * 10;
            start_col -= 1;
        }
        ratio.ratio[i] = number;
    }

    return ratio;
}
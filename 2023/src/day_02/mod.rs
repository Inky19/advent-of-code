use std::fs;
use regex::Regex;

pub fn run(){
    let file = fs::read_to_string("../data/2023/day_02/input.txt")
    .expect("Could not read input.txt");
    let lines:Vec<_> = file.lines().collect();
    pb1and2(&lines);
}

fn pb1and2(lines:&[&str]){
    let re_game_r = Regex::new("[0-9]+ red").unwrap();
    let re_game_g = Regex::new("[0-9]+ green").unwrap();
    let re_game_b = Regex::new("[0-9]+ blue").unwrap();
    let re_game_rgb:[Regex;3] = [re_game_r, re_game_g, re_game_b];
    let max_set_pb1:[u32; 3] = [12, 13, 14];
    let mut max_set_pb2:[u32; 3];

    let mut sum1 = 0;
    let mut sum2 = 0;
    for l in 0..lines.len() {
        let mut correct = true;
        max_set_pb2 = [0,0,0];
        for i in 0..3 {
            let max_col = reg_max(&re_game_rgb[i], lines[l]);
            if  max_col > max_set_pb1[i] {
                correct = false;
                //break; // Can't break for pb2
            }
            if max_set_pb2[i] < max_col {
                max_set_pb2[i] = max_col;
            }
        }
        if correct{
            sum1 += l+1;
        }
        sum2 += max_set_pb2[0]*max_set_pb2[1]*max_set_pb2[2];
    }
    println!("\n====== Problem 1 ======");
    println!("{}", sum1);
    println!("\n====== Problem 2 ======");
    println!("{}", sum2);
}

fn reg_max(reg:&Regex, str:&str) -> u32{
    let reg_num = Regex::new("[0-9]+").unwrap();
    let extract: Vec<&str> = reg.find_iter(str).map(|m| m.as_str()).collect();
    let mut sum  = 0;
    let mut tmp;
    for s in extract {
        tmp = reg_num.find(&s).map(|m| m.as_str()).unwrap().parse::<u32>().unwrap();
        if tmp > sum {sum = tmp;}
    }
    return sum;
}
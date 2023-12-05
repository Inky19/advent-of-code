use std::fs;

pub fn run(){
    let file = fs::read_to_string("../data/2023/day_04/input.txt")
    .expect("Could not read input.txt");
    let lines:Vec<_> = file.lines().collect();
    println!("\n====== Problem 1 ======");
    pb1(&lines);
    println!("\n====== Problem 2 ======");
    pb2(&lines);
}

fn pb1(lines:&[&str]){
    let mut sum = 0;
    for line in lines {
        let mut power = 0;
        let lists:Vec<_> = line.split(':').collect();
        let mut lists = lists[1].split(" | ");
        let winning_numbers:Vec<_> = lists.next().unwrap().split(" ").filter(|x| *x!="").map(|x| x.parse::<u32>().unwrap()).collect();
            
        let numbers:Vec<_> = lists.next().unwrap().split(" ").filter(|x| *x!="").map(|x| x.parse::<u32>().unwrap()).collect();
        for num in numbers {
            if winning_numbers.contains(&num) {
                power +=1;
            }
        }
        if power!=0 {
            sum += 1<<power-1;
        }
        
    }
    println!("{}", sum);
}

fn pb2(lines:&[&str]){
    let mut subcards:Vec<u32> = Vec::new();
    for line in lines {
        let mut num_subcards = 0;
        let lists:Vec<_> = line.split(':').collect();
        let mut lists = lists[1].split(" | ");
        let winning_numbers:Vec<_> = lists.next().unwrap().split(" ").filter(|x| *x!="").map(|x| x.parse::<u32>().unwrap()).collect();
            
        let numbers:Vec<_> = lists.next().unwrap().split(" ").filter(|x| *x!="").map(|x| x.parse::<u32>().unwrap()).collect();
        for num in numbers {
            if winning_numbers.contains(&num) {
                num_subcards +=1;
            }
        }
        subcards.push(num_subcards);
    }

    let mut cards:Vec<u32> = Vec::new();
    let mut new_cards:Vec<u32> = Vec::new();
    let mut cards_tmp = Vec::new();

    // Original set of cards
    for i in 0..lines.len() {
        new_cards.push(i as u32);
    }
    

    let mut done = false;
    while !done {
        done = true;
        
        for card in &new_cards {
            if done && subcards[*card as usize] != 0 {
                done = false;
            }
            for i in 1..=subcards[*card as usize] {
                cards_tmp.push(card+i);
            }
        }
        cards.append(&mut new_cards);
        new_cards = cards_tmp;
        cards_tmp = Vec::new();
    }

    println!("{}", cards.len());
}
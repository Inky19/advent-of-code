use std::io;

mod day_01;
mod day_02;
mod day_03;

fn main() {
    loop {
        let mut day = String::new();
        println!("\nEnter the number of the day:");
        io::stdin()
            .read_line(&mut day)
            .expect("Failed to read line");

        let day: u8 = match day.trim().parse() {
            Ok(val) => val,
            Err(_) => {
                println!("Invalid input!");
                continue;
            }
        };

        match day {
            1 => day_01::run(),
            2 => day_02::run(),
            3 => day_03::run(),
            5_u8..=24_u8 => println!("Not implemented yet"),
            _ => println!("Number must be between 1 and 24!"),
        }
    }
}

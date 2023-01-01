pub mod days;
mod helpers;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let day: u8 = args[1]
        .parse()
        .unwrap_or_else(|v| panic!("Day is invalid: {}", v));

    call_day(day, args.len() == 3 && args[2] == "cool");
}

fn call_day(day: u8, cool: bool) {
    if cool {
        match day {
            6 => days::day6::cool(),
            _ => unimplemented!(),
        }
    } else {
        match day {
            1 => days::day1::solution(),
            2 => days::day2::solution(),
            3 => days::day3::solution(),
            4 => days::day4::solution(),
            5 => days::day5::solution(),
            6 => days::day6::solution(),
            7 => days::day7::solution(),
            8 => days::day8::solution(),
            _ => unimplemented!(),
        }
    }
}

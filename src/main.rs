pub mod days;
mod helpers;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let day: u8 = args[1]
        .parse()
        .unwrap_or_else(|v| panic!("Day is invalid: {}", v));

    call_day(day)
}

fn call_day(day: u8) {
    match day {
        1 => days::day1::solution(),
        2 => days::day2::solution(),
        3 => days::day3::solution(),
        _ => unimplemented!(),
    }
}

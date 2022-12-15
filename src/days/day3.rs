use crate::helpers::reader::read_file;
use std::collections::HashSet;

pub fn solution() {
    let string = read_file("3");
    let lines: Vec<&str> = string.trim_end().split('\n').collect();

    let part_1_score = get_part_1_score(&lines);
    let part_2_score = get_part_2_score(&lines);
    println!("Part 1: {part_1_score}");
    println!("Part 2: {part_2_score}");
}

fn get_char_priority(char: char) -> i32 {
    let decimal_byte_value: i32 = char.to_string().as_bytes()[0].into(); // the into() converts the variable into the specified type, in this case i32.
    let priority: i32 = if char.is_uppercase() {
        decimal_byte_value - 38
    } else {
        decimal_byte_value - 96
    };
    priority
}

fn get_part_1_score(lines: &Vec<&str>) -> i32 {
    let mut score: i32 = 0; // cannot be a u8 (the default) because that is max 255.
    for line in lines {
        let common_char = get_part_1_common_char(line);
        let priority: i32 = get_char_priority(common_char);
        score += priority;
    }
    score
}

fn get_part_1_common_char(line: &str) -> char {
    let split_idx = line.len() / 2;
    let first_half = &line[..split_idx];
    let second_half = &line[split_idx..];

    let set: HashSet<char> = first_half.chars().collect();
    return second_half
        .chars()
        .find(|x| set.contains(x))
        .expect("There is no common character!");
}

fn get_part_2_score(lines: &Vec<&str>) -> i32 {
    let mut score: i32 = 0;
    for i in (0..lines.len()).step_by(3) {
        let common_char = get_part_2_common_char(&lines[i..i + 3]);
        let priority: i32 = get_char_priority(common_char);
        score += priority;
    }
    score
}

fn get_part_2_common_char(group: &[&str]) -> char {
    let line_1_set: HashSet<char> = group[0].chars().collect();
    let line_2_set: HashSet<char> = group[1].chars().collect();
    return group[2]
        .chars()
        .find(|x| line_1_set.contains(x) && line_2_set.contains(x))
        .expect("There is no common character in the group!");
}

// Strategy: get utf-8 decimal of character (utf-8 uses bytes but we get decimal version)
// https://asecuritysite.com/coding/asc2
// Upper case = 65-90
// Uppercase item types A through Z have priorities 27 through 52.
// therefore: utf-8 value of char minus 38

// Lower case = 97-122
// Lowercase item types a through z have priorities 1 through 26.
// therefore: utf-8 value of char minus 96

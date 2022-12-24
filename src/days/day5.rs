use crate::helpers::reader::read_file;
use regex::Regex;
use std::collections::HashMap;

pub fn solution() {
    let string = read_file("5");
    let mut split_iter = string.split("\n\n");

    let first_half: Vec<&str> = split_iter
        .next()
        .expect("No first half")
        .split("\n")
        .collect();
    let second_half: Vec<&str> = split_iter
        .next()
        .expect("No second half")
        .split("\n")
        .collect();
    let stacks = get_stacks_hashmap(first_half);
    let part_1_solution = get_part_1_solution(stacks.clone(), &second_half);
    let part_2_solution = get_part_2_solution(stacks.clone(), &second_half);
    println!("Part 1: {part_1_solution}");
    println!("Part 2: {part_2_solution}");
}

fn get_part_1_solution(mut stacks: HashMap<char, String>, lines: &Vec<&str>) -> String {
    for line in lines {
        let instructions = get_instructions(line);

        let stack_to_take_from_idx = &char::from_digit(instructions[1], 10).unwrap();
        let stack_to_take_from = stacks
            .get(stack_to_take_from_idx)
            .expect("Should be a string to move");

        let stack_to_add_to_idx = &char::from_digit(instructions[2], 10).unwrap();
        let stack_to_add_to = stacks
            .get(stack_to_add_to_idx)
            .expect("Should be a string to move");

        let split_idx = stack_to_take_from.len() - (instructions[0] as usize);
        let first_part = &stack_to_take_from[..split_idx];
        let second_part = &stack_to_take_from[split_idx..];
        let reversed = second_part.chars().rev().collect::<String>();
        let new_stack = format!("{}{}", stack_to_add_to, reversed);

        // the ordering below is important - we have to save stack_to_take_from that before we do operations on stack_to_add_to - otherwise we aren't sure if the stack_to_take_from data is correct when we try to save stack_to_take_from(?)
        stacks.insert(*stack_to_take_from_idx, first_part.to_string());
        stacks.insert(*stack_to_add_to_idx, new_stack);
    }

    let result = get_last_letters_of_stacks(stacks);
    result
}

fn get_part_2_solution(mut stacks: HashMap<char, String>, lines: &Vec<&str>) -> String {
    for line in lines {
        let instructions = get_instructions(line);

        let stack_to_take_from_idx = &char::from_digit(instructions[1], 10).unwrap();
        let stack_to_take_from = stacks
            .get(stack_to_take_from_idx)
            .expect("Should be a string to move");

        let stack_to_add_to_idx = &char::from_digit(instructions[2], 10).unwrap();
        let stack_to_add_to = stacks
            .get(stack_to_add_to_idx)
            .expect("Should be a string to move");

        let split_idx = stack_to_take_from.len() - (instructions[0] as usize);
        let first_part = &stack_to_take_from[..split_idx];
        let second_part = &stack_to_take_from[split_idx..];
        let new_stack = format!("{}{}", stack_to_add_to, second_part);

        // the ordering below is important - we have to save stack_to_take_from that before we do operations on stack_to_add_to - otherwise we aren't sure if the stack_to_take_from data is correct when we try to save stack_to_take_from(?)
        stacks.insert(*stack_to_take_from_idx, first_part.to_string());
        stacks.insert(*stack_to_add_to_idx, new_stack);
    }

    let result = get_last_letters_of_stacks(stacks);
    result
}

fn get_stacks_hashmap(lines: Vec<&str>) -> HashMap<char, String> {
    let mut stacks_map: HashMap<char, String> = HashMap::new();

    let last_line = lines.last().expect("Can't get last");
    let last_line_len = last_line.len();

    for line_idx in (1..last_line_len).step_by(4) {
        let stack_num = last_line.as_bytes()[line_idx] as char;

        for line_num in (0..(lines.len() - 1)).rev() {
            let char = lines[line_num].as_bytes()[line_idx] as char;
            if char.is_alphabetic() {
                stacks_map
                    .entry(stack_num)
                    .and_modify(|x| x.push(char))
                    .or_insert(char.to_string());
            }
        }
    }
    stacks_map
}

fn get_instructions(line: &str) -> Vec<u32> {
    let reg = Regex::new(r"\d+").unwrap();

    return reg
        .find_iter(line)
        .map(|n| {
            n.as_str()
                .parse::<u32>()
                .expect("Instruction should be a parseable number")
        })
        .collect();
}

fn get_last_letters_of_stacks(stacks: HashMap<char, String>) -> String {
    let mut result = String::from("");
    let stacks_size = stacks.keys().len();
    for idx in 1..stacks_size + 1 {
        let stack_idx = &char::from_digit(idx as u32, 10).unwrap();
        let last_letter = stacks
            .get(stack_idx)
            .expect("Should be a stack index")
            .chars()
            .last()
            .expect("Should be a char");
        result.push(last_letter);
    }
    result
}

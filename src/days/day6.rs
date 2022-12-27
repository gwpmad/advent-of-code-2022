use crate::helpers::reader::read_file;
use literal::{set, SetLiteral};
use std::collections::HashSet;

pub fn solution() {
    let string = read_file("6");
    let bytes = string.as_bytes();
    let part_1_solution = get_part_1_solution(bytes);
    let part_2_solution = get_part_2_solution(string);
    println!("Part 1: {part_1_solution}");
    println!("Part 2: {part_2_solution}");
}

fn get_part_1_solution(bytes: &[u8]) -> i32 {
    let mut result = 0;

    for i in 0..bytes.len() - 4 {
        let set: HashSet<char> =
            set! { bytes[i] as char, bytes[i+1] as char, bytes[i+2] as char, bytes[i+3] as char };

        if set.len() == 4 {
            result = i + 4;
            break;
        }
    }
    return result as i32;
}

fn get_part_2_solution(string: String) -> i32 {
    let mut result = 0;

    for i in 0..string.as_bytes().len() - 14 {
        let slice = &string[i..i + 14];
        let set: HashSet<char> = HashSet::from_iter(slice.chars());

        if set.len() == 14 {
            result = i + 14;
            break;
        }
    }
    return result as i32;
}

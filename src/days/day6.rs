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

// Copied from https://www.reddit.com/r/adventofcode/comments/zdw0u6/comment/iz42lhq/?utm_source=reddit&utm_medium=web2x&context=3
// For interest. This is a sliding window problem and below appears to be a very efficient solution (I think it's o(n) whatever the marker size)
pub fn cool() {
    let string = read_file("6");
    let input: Vec<char> = string.chars().collect();
    const MARKER_SIZE: usize = 14;
    let mut occurrences = [0u16; 256]; // this doesn't need to be 256 long. It's an array of zeroes that represent the utf encoding of the letters e.g. m = 109
    let mut different_letters = 0;
    println!(
        "{}",
        input
            .iter()
            .enumerate()
            .find_map(|(n, &i)| {
                println!("\n\nn: {n} i: {i}");
                println!("occurences at beginning {:?}", occurrences);
                println!("different_letters at beginning {:?}", different_letters);
                if n >= MARKER_SIZE {
                    println!("n >= MARKER SIZE");
                    println!(
                        "decrementing occurrences[input[n - MARKER_SIZE]] ({:?}) to {:?}",
                        input[n - MARKER_SIZE],
                        occurrences[input[n - MARKER_SIZE] as usize] - 1
                    );
                    occurrences[input[n - MARKER_SIZE] as usize] -= 1;
                    if occurrences[input[n - MARKER_SIZE] as usize] == 0 {
                        println!("occurrences[input[n - MARKER_SIZE] as usize] == 0");
                        different_letters -= 1; // the letter MARKER_SIZE ago is no longer present in the sliding window so decrement different_letters
                        println!(
                            "decremented different_letters in nested if to: {}",
                            different_letters
                        );
                    }
                }
                println!("i as usize {}", i as usize);
                if occurrences[i as usize] == 0 {
                    println!("occurrences[i as usize] == 0");
                    different_letters += 1; // because it was not present in occurrences before, this is a new letter so increment different_letters
                    println!("different_letters raised to: {}", different_letters);
                }
                occurrences[i as usize] += 1;
                println!(
                    "occurrences after incrementing of {i} (this always happens): {:?}",
                    occurrences
                );
                if different_letters == MARKER_SIZE {
                    println!("different_letters == MARKER_SIZE");
                    Some(n + 1) // There are MARKER_SIZE different letters so the answer has been found
                } else {
                    None
                }
            })
            .unwrap()
    );
}

use crate::helpers::reader::read_file;

pub fn solution() {
    let string = read_file("1");
    let string_without_last_char = &string[0..&string.len() - 1];
    let calories_vec: Vec<Vec<i32>> = string_without_last_char
        .split("\n\n")
        .map(|s| {
            s.split('\n')
                .map(|num| {
                    num.parse::<i32>()
                        .unwrap_or_else(|v| panic!("Num is invalid: {}", v))
                })
                .collect()
        })
        .collect();

    let mut sums: Vec<i32> = calories_vec
        .iter()
        .map(|calories| calories.iter().sum::<i32>())
        .collect();

    sums.sort_by(|a, b| b.cmp(a));

    println!("Part 1: {:?}", sums[0]);
    println!("Part 2: {:?}", sums[0] + sums[1] + sums[2]);
}

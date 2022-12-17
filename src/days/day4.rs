use crate::helpers::reader::read_file;

pub fn solution() {
    let string = read_file("4");
    let lines: Vec<&str> = string.trim_end().split('\n').collect();

    let part_1_solution = get_part_1_solution(&lines);
    let part_2_solution = get_part_2_solution(&lines);
    println!("Part 1: {part_1_solution}");
    println!("Part 2: {part_2_solution}");
}

fn get_part_1_solution(lines: &Vec<&str>) -> i32 {
    let mut score: i32 = 0;

    for line in lines {
        let sets = get_sets(&line);
        if (sets.0[0] <= sets.1[0] && sets.0[1] >= sets.1[1])
            || (sets.1[0] <= sets.0[0] && sets.1[1] >= sets.0[1])
        {
            score += 1;
        }
    }
    score
}

fn get_part_2_solution(lines: &Vec<&str>) -> i32 {
    let mut score: i32 = 0;

    for line in lines {
        let sets = get_sets(&line);
        if sets_overlap(sets.0, sets.1) {
            score += 1;
        }
    }
    score
}

fn sets_overlap(set_1: Vec<i32>, set_2: Vec<i32>) -> bool {
    // if either of the two other numbers is within that a set (inclusive) - true
    let mut result = false;
    for n in 0..2 {
        if set_2[0] <= set_1[n] && set_1[n] <= set_2[1] {
            result = true;
        }
        if set_1[0] <= set_2[n] && set_2[n] <= set_1[1] {
            result = true;
        }
    }
    result
}

fn get_sets(line: &str) -> (Vec<i32>, Vec<i32>) {
    let ranges: Vec<&str> = line.split(",").collect();
    let set_1: Vec<i32> = ranges[0]
        .split("-")
        .map(|n| n.parse::<i32>().expect("Couldn't parse int"))
        .collect();

    let set_2: Vec<i32> = ranges[1]
        .split("-")
        .map(|n| n.parse::<i32>().expect("Couldn't parse int"))
        .collect();

    (set_1, set_2)
}

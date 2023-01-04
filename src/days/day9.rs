use crate::helpers::reader::read_file;
use std::collections::{HashMap, HashSet};

pub fn solution() {
    let string = read_file("9");
    let lines: Vec<&str> = string.split('\n').collect();
    let part_1_solution = move_knots(&lines, 1);
    let part_2_solution = move_knots(&lines, 9);
    println!("Part 1: {part_1_solution}");
    println!("Part 2: {part_2_solution}");
}

fn move_knots(lines: &Vec<&str>, number_of_followers: usize) -> usize {
    let mut knots = get_knots(number_of_followers);
    let mut tail_visited = HashSet::<(i32, i32)>::new();

    for line in lines {
        let split_direction: Vec<&str> = line.split(' ').collect();
        let distance = split_direction[1].parse::<i32>().expect("Should be an int");

        for _ in 0..distance {
            move_head_knot(&mut knots, split_direction[0]);
            for leader_idx in 0..number_of_followers {
                move_follower(&mut knots, leader_idx);
            }
            tail_visited.insert(*knots.get(&number_of_followers).unwrap());
        }
    }
    tail_visited.len()
}

fn move_head_knot(knots: &mut HashMap<usize, (i32, i32)>, direction: &str) {
    knots.entry(0).and_modify(|head_knot| match direction {
        "R" => head_knot.0 += 1,
        "L" => head_knot.0 -= 1,
        "U" => head_knot.1 += 1,
        "D" => head_knot.1 -= 1,
        _ => println!("Wrong direction: {}", direction),
    });
}

fn move_follower(knots: &mut HashMap<usize, (i32, i32)>, leader_idx: usize) {
    let follower_idx = leader_idx + 1;
    let movements = get_movements(
        knots.get(&leader_idx).unwrap(),
        knots.get(&follower_idx).unwrap(),
    );
    knots.entry(follower_idx).and_modify(|knot| {
        knot.0 += movements.0;
        knot.1 += movements.1;
    });
}

fn get_movements(leader_after_move: &(i32, i32), follower_before_move: &(i32, i32)) -> (i32, i32) {
    let abs_delta = (
        ((leader_after_move.0 - follower_before_move.0).abs()),
        ((leader_after_move.1 - follower_before_move.1).abs()),
    );
    let mut movements: (i32, i32) = (
        leader_after_move.0 - follower_before_move.0,
        leader_after_move.1 - follower_before_move.1,
    );
    if abs_delta.0 != 2 && abs_delta.1 != 2 {
        movements = (0, 0)
    }
    if abs_delta.0 == 2 {
        movements.0 = movements.0 / 2;
    }
    if abs_delta.1 == 2 {
        movements.1 = movements.1 / 2;
    }

    movements
}

fn get_knots(number_of_followers: usize) -> HashMap<usize, (i32, i32)> {
    let mut knots: HashMap<usize, (i32, i32)> = HashMap::new();
    knots.insert(0, (0, 0));
    for i in 0..number_of_followers {
        knots.insert(i + 1, (0, 0));
    }
    knots
}

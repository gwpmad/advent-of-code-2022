use crate::helpers::reader::read_file;
use std::collections::HashMap;

pub fn solution() {
    let string = read_file("2");

    let lines: Vec<&str> = string.trim_end().split('\n').collect();

    let part_1_score = get_part_1_score(&lines, get_outcome_lookup(), get_shape_score_lookup());
    let part_2_score = get_part_2_score(
        &lines, // need to send 'borrowed' lines value both times because Vec doesn't implement Copy, so without this lines becomes 'owned' by get_part_1_score
        get_required_move_lookup(),
        get_outcome_lookup(),
        get_shape_score_lookup(),
    );
    println!("Part 1: {part_1_score}");
    println!("Part 2: {part_2_score}");
}

fn get_moves(line: &str) -> (char, char) {
    let first_move = line
        .chars()
        .next()
        .expect("Should have been a char here - first move");
    let second_move = line
        .chars()
        .nth(line.len() - 1)
        .expect("Should have been a char here - second move");
    (first_move, second_move)
}

fn get_part_1_score(
    lines: &Vec<&str>,
    outcome_lookup: HashMap<(char, char), i32>,
    shape_score_lookup: HashMap<char, i32>,
) -> i32 {
    let mut score = 0;
    for line in lines {
        let moves = get_moves(line);

        let round_score = outcome_lookup
            .get(&moves)
            .expect("Round score should be an i32");
        let shape_score = shape_score_lookup
            .get(&moves.1)
            .expect("Shape score should be an i32");

        score += round_score;
        score += shape_score;
    }
    score
}

fn get_part_2_score(
    lines: &Vec<&str>,
    required_move_lookup: HashMap<(char, char), char>,
    outcome_lookup: HashMap<(char, char), i32>,
    shape_score_lookup: HashMap<char, i32>,
) -> i32 {
    let mut score = 0;
    for line in lines {
        let moves = get_moves(line);

        let required_move = required_move_lookup
            .get(&moves)
            .expect("Required move should be a char");
        // println!("{}", moves.1);
        // println!("{}", required_move);
        let round_score = outcome_lookup
            .get(&(moves.0, required_move.clone()))
            .expect("Round score should be an i32");
        let shape_score = shape_score_lookup
            .get(required_move)
            .expect("Shape score should be an i32");

        score += round_score;
        score += shape_score;
    }
    score
}

fn get_outcome_lookup() -> HashMap<(char, char), i32> {
    let win = 6;
    let draw = 3;
    let loss = 0;

    let mut lookup: HashMap<(char, char), i32> = HashMap::new();
    lookup.insert(('A', 'X'), draw);
    lookup.insert(('A', 'Y'), win);
    lookup.insert(('A', 'Z'), loss);

    lookup.insert(('B', 'X'), loss);
    lookup.insert(('B', 'Y'), draw);
    lookup.insert(('B', 'Z'), win);

    lookup.insert(('C', 'X'), win);
    lookup.insert(('C', 'Y'), loss);
    lookup.insert(('C', 'Z'), draw);
    lookup
}

fn get_shape_score_lookup() -> HashMap<char, i32> {
    let mut lookup: HashMap<char, i32> = HashMap::new();

    lookup.insert('X', 1);
    lookup.insert('Y', 2);
    lookup.insert('Z', 3);
    lookup
}

fn get_required_move_lookup() -> HashMap<(char, char), char> {
    let mut lookup: HashMap<(char, char), char> = HashMap::new();

    lookup.insert(('A', 'X'), 'Z');
    lookup.insert(('A', 'Y'), 'X');
    lookup.insert(('A', 'Z'), 'Y');

    lookup.insert(('B', 'X'), 'X');
    lookup.insert(('B', 'Y'), 'Y');
    lookup.insert(('B', 'Z'), 'Z');

    lookup.insert(('C', 'X'), 'Y');
    lookup.insert(('C', 'Y'), 'Z');
    lookup.insert(('C', 'Z'), 'X');

    lookup
}

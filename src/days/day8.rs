use crate::helpers::reader::read_file;
use std::collections::{HashMap, HashSet};

pub fn solution() {
    let string = read_file("8");
    let lines: Vec<Vec<i32>> = string
        .trim_end()
        .split('\n')
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse::<i32>().expect("Couldn't parse int"))
                .collect()
        })
        .collect();
    let part_1_solution = get_part_1_solution(&lines);
    let part_2_solution = get_part_2_solution(&lines);
    println!("Part 1: {part_1_solution}");
    println!("Part 2: {part_2_solution}");
}

fn get_part_1_solution(lines: &Vec<Vec<i32>>) -> usize {
    let mut visible_trees = HashSet::<String>::new();

    for (vertical_pos, line) in lines.iter().enumerate() {
        record_trees_visibility(&mut visible_trees, line, vertical_pos as i32, false);
    }

    for horizontal_pos in 0..lines.len() {
        let line: Vec<i32> = lines.iter().map(|vec| vec[horizontal_pos]).collect();
        record_trees_visibility(&mut visible_trees, &line, horizontal_pos as i32, true);
    }
    visible_trees.len()
}

fn get_part_2_solution(lines: &Vec<Vec<i32>>) -> usize {
    let mut tree_scenic_scores = HashMap::<String, usize>::new();

    for (vertical_pos, line) in lines.iter().enumerate() {
        record_trees_scenic_scores(&mut tree_scenic_scores, line, vertical_pos as i32, false);
    }

    for horizontal_pos in 0..lines.len() {
        let line: Vec<i32> = lines.iter().map(|vec| vec[horizontal_pos]).collect();
        record_trees_scenic_scores(&mut tree_scenic_scores, &line, horizontal_pos as i32, true);
    }
    tree_scenic_scores.iter().fold(0, |max, entry| {
        if *entry.1 > max {
            return *entry.1;
        }
        max
    })
}

struct DirectionalScoreEntry {
    score: usize,
    finished: bool,
}

fn record_trees_scenic_scores(
    tree_scenic_scores: &mut HashMap<String, usize>,
    line: &Vec<i32>,
    constant_pos: i32,
    is_vertical: bool,
) {
    let mut directional_scores = HashMap::<i32, DirectionalScoreEntry>::new();

    for (iterator_pos, height) in line.iter().enumerate() {
        record_directional_scores_at_current_pos(
            &mut directional_scores,
            iterator_pos as i32,
            *height,
            line,
        );
    }

    flush_directional_scores(
        &mut directional_scores,
        tree_scenic_scores,
        constant_pos,
        is_vertical,
    );

    let offset: i32 = line.len() as i32 - 1;
    for (iterator_pos, height) in line.iter().rev().enumerate() {
        record_directional_scores_at_current_pos(
            &mut directional_scores,
            offset - iterator_pos as i32,
            *height,
            line,
        );
    }

    flush_directional_scores(
        &mut directional_scores,
        tree_scenic_scores,
        constant_pos,
        is_vertical,
    );
}

fn record_directional_scores_at_current_pos(
    directional_scores: &mut HashMap<i32, DirectionalScoreEntry>,
    current_pos: i32,
    height: i32,
    whole_line: &Vec<i32>,
) {
    directional_scores
        .iter_mut()
        .filter(|score_entry| !score_entry.1.finished)
        .for_each(|score_entry| {
            score_entry.1.score += 1;
            if height >= whole_line[*score_entry.0 as usize] {
                score_entry.1.finished = true;
            }
        });
    directional_scores.insert(
        current_pos,
        DirectionalScoreEntry {
            score: 0,
            finished: false,
        },
    );
}

fn flush_directional_scores(
    directional_scores: &mut HashMap<i32, DirectionalScoreEntry>,
    tree_scenic_scores: &mut HashMap<String, usize>,
    constant_pos: i32,
    is_vertical: bool,
) {
    directional_scores.iter().for_each(|score_entry| {
        let stamp = get_stamp(constant_pos, *score_entry.0 as i32, is_vertical);
        tree_scenic_scores
            .entry(stamp)
            .and_modify(|scenic_score| *scenic_score *= score_entry.1.score)
            .or_insert(score_entry.1.score);
    });

    directional_scores.clear();
}

fn record_trees_visibility(
    visible_trees: &mut HashSet<String>,
    line: &Vec<i32>,
    constant_pos: i32,
    is_vertical: bool,
) {
    let mut tallest: i32 = -1;
    for (iterator_pos, e) in line.iter().enumerate() {
        if *e > tallest {
            let stamp = get_stamp(constant_pos, iterator_pos as i32, is_vertical);
            visible_trees.insert(stamp);
            tallest = *e;
        }
    }
    tallest = -1;
    let offset: i32 = line.len() as i32 - 1;
    for (iterator_pos, e) in line.iter().rev().enumerate() {
        if *e > tallest {
            let stamp = get_stamp(constant_pos, offset - iterator_pos as i32, is_vertical);
            visible_trees.insert(stamp);
            tallest = *e;
        }
    }
}

fn get_stamp(pos1: i32, pos2: i32, reverse_positions: bool) -> String {
    let stamp = if reverse_positions {
        format!("{},{}", pos2, pos1)
    } else {
        format!("{},{}", pos1, pos2)
    };
    stamp
}

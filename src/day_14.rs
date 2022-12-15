use std::collections::HashSet;
use itertools::{max, min};
use regex::Regex;
use crate::read_input;

fn get_blockers() -> HashSet<(i32, i32)> {
    let input = read_input::read("inputs/14.txt");
    let mut blockers: HashSet<(i32, i32)> = HashSet::new();
    for line in input.lines() {
        let mut points: Vec<(i32, i32)> = vec![];
        let re = Regex::new(r"(\d+),(\d+)").unwrap();
        for cap in re.captures_iter(line) {
            let x = cap[1].parse::<i32>().unwrap();
            let y = cap[2].parse::<i32>().unwrap();
            points.push((x, y))
        }
        for i in 0..points.len()-1 {
            let (x1, y1) = points.get(i).unwrap();
            let (x2, y2) = points.get(i+1).unwrap();
            if x1 == x2 {
                for j in *min([y1, y2]).unwrap()..*max([y1, y2]).unwrap() + 1 {
                    blockers.insert((*x1, j));
                }
            } else {
                for j in *min([x1, x2]).unwrap()..*max([x1, x2]).unwrap() + 1 {
                    blockers.insert((j, *y1));
                }
            }
        }
    }
    blockers

}

fn get_sand_grain_count(mut blockers: HashSet<(i32, i32)>) -> i32 {
    let max_y = &blockers
        .iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .unwrap().1 + 1;
    let mut count = 0;
    let mut pos: (i32, i32) = (500, 0);
    let mut path_to_date: Vec<(i32, i32)> = vec![pos];
    loop {
        path_to_date.push(pos);
        if pos.1 >= max_y || blockers.contains(&(500, 0)) {
            break;
        } else if !blockers.contains(&(pos.0, pos.1 + 1)) {
            pos = (pos.0, pos.1 + 1);
        } else if !blockers.contains(&(pos.0 - 1, pos.1 + 1)) {
            pos = (pos.0 - 1, pos.1 + 1);
        } else if !blockers.contains(&(pos.0 + 1, pos.1 + 1)) {
            pos = (pos.0 + 1, pos.1 + 1);
        } else {
            count += 1;
            blockers.insert(pos);
            let _settled_position = path_to_date.pop().unwrap();
            let previous_position = path_to_date.pop().unwrap();
            pos = previous_position;
        }
    }
    count
}

fn part_1() {
    let blockers = get_blockers();
    let count = get_sand_grain_count(blockers);
    println!("Part 1: {}", count);
}

fn part_2() {
    let mut blockers = get_blockers();
    let max_y = &blockers
        .iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .unwrap().1 + 2;
    for i in (-2 * max_y)..(2*max_y) {
        blockers.insert((500+i, max_y.clone()));
    }
    let count = get_sand_grain_count(blockers);
    println!("Part 2: {}", count);
}

pub fn solve() {
    println!("Day 14");
    part_1();
    part_2();
    println!();
}

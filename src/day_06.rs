use std::collections::HashSet;
use crate::read_input;


fn find_index_of_unique_character_set(threshold: usize) -> usize {
    let input = read_input::read("inputs/06.txt");
    for i in threshold..input.len() {
        if HashSet::<char>::from_iter(input[i-&threshold..i].chars()).len() < threshold {
            return i
        }
    }
    panic!("Did not find marker in data");
}

fn part_1() {
    println!("Part 1: {}", find_index_of_unique_character_set(4).to_string());
}

fn part_2() {
    println!("Part 2: {}", find_index_of_unique_character_set(14).to_string());
}

pub fn solve() {
    println!("Day 06");
    part_1();
    part_2();
    println!();
}

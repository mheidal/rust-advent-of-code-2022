use std::collections::HashSet;
use crate::read_input;


fn find_index_of_unique_character_set(threshold: usize) -> usize {
    let mut buffer: Vec<char> = vec!['a'; threshold];
    let mut buffer_index: usize = 0;
    let input = read_input::read("inputs/06.txt");
    for (input_index, ch) in input.chars().enumerate() {
        if &input_index < &threshold || HashSet::<&char>::from_iter(&buffer[..]).len() < threshold {
            buffer[buffer_index] = ch;
            buffer_index = (&buffer_index + 1) % &threshold
        } else {
            return input_index;
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

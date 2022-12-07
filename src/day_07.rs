use std::collections::HashMap;
use crate::read_input;

fn get_dir_sizes() -> Vec<i32> {
    let input = read_input::read("inputs/07.txt");
    let mut dirs: HashMap<String, i32> = HashMap::new();
    let mut current: Vec<String> = vec![String::from("/")];
    for line in input.lines() {
        match line.trim().split(' ').collect::<Vec<&str>>()[..] {
            ["$", "cd", destination_symbol] => {
                match destination_symbol {
                    "/" => current = vec![String::from("/")],
                    ".." => current.truncate(current.len() - 1),
                    dir_name => {
                        current.push(String::from(dir_name))
                    }
                };
            },
            ["$", "ls"] => continue,
            ["dir", _] => continue,
            [size, _] => {
                let size = size.parse::<i32>().expect("Valid size");
                for i in 0..current.len() {
                    let dir_index = current[0..i+1].join("_");
                    *dirs.entry(dir_index).or_insert(0) += size;
                }
            },
            _ => panic!("Unexpected value"),
        }
    }
    dirs.values().map(|i| *i).collect()

}

fn part_1() {
    let sum: i32 = get_dir_sizes()
        .into_iter()
        .filter(|i| *i <= 100_000)
        .sum();

    println!("Part 1: {}", sum);
}

fn part_2() {
    let file_system_size = 70_000_000;
    let space_required = 30_000_000;
    let dir_sizes = get_dir_sizes();
    let free_space = file_system_size - dir_sizes[..]
        .into_iter()
        .max()
        .expect("Root folder size");
    let smallest_sufficient_folder_size = dir_sizes
        .into_iter()
        .filter(|i| *i >= (space_required - free_space))
        .min()
        .expect("At least one sufficient folder");
    println!("Part 2: {}", smallest_sufficient_folder_size);
}

pub fn solve() {
    println!("Day 07");
    part_1();
    part_2();
    println!();
}

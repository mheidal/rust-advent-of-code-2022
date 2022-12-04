use crate::read_input;

fn get_priority(item: char) -> i32 {
    let ascii_val = item as i32;
    match ascii_val {
        d if (65 <= d && 96 >= d) => d - 38,
        d if (97 <= d) => d - 96,
        _ => panic!("Character wasn't alphabetical!")
    }
}

fn get_unique_chars_in_str(string: &str) -> Vec<char> {
    let mut chars: Vec<char> = Vec::new();
    for character in string.chars() {
        if !chars.contains(&character) {
            chars.push(character)
        }
    }
    chars
}

fn part_1() {
    let input = read_input::read("inputs/03.txt");
    let mut summed_priorities = 0;
    for line in input.lines() {
        let trimmed_line = line.trim();
        let line_len = trimmed_line.len();
        let first_compartment = &trimmed_line[..line_len/2];
        let second_compartment = &trimmed_line[line_len/2..];
        for item in get_unique_chars_in_str(first_compartment) {
            if second_compartment.contains(item) {
                summed_priorities += get_priority(item);
            }
        }
    }
    println!("Part 1: {}", summed_priorities);
}

fn part_2() {
    let input = read_input::read("inputs/03.txt");
    let mut summed_priorities = 0;
    let mut has_next = true;
    let mut lines = input.lines();
    while has_next {
        match lines.next() {
            Some(elf_1) => {
                let elf_2 = lines.next().expect("Expected second line in triplet").trim();
                let elf_3 = lines.next().expect("Expected third line in triplet").trim();
                for item in get_unique_chars_in_str(elf_1.trim()) {
                    if elf_2.contains(item) && elf_3.contains(item) {
                        summed_priorities += get_priority(item);
                    }
                }
            },
            None => has_next = false,
        }
    }
    println!("Part 2: {}", summed_priorities);
}

pub fn solve() {
    println!("Day 03");
    part_1();
    part_2();
    println!();
}

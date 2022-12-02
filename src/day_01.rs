use crate::read_input;

fn get_list_of_elves() -> Vec<i32> {
    let mut elves: Vec<i32> = Vec::new();
    let input = read_input::read("inputs/01.txt");
    let mut val = 0;
    for elf in input.lines() {
        if elf.trim().len() > 0 {
            val += elf
                .trim()
                .parse::<i32>()
                .expect("Integer");
        } else {
            elves.push(val);
            val = 0;
        }
    }
    elves.sort();
    elves
}

fn part_1() {
    let elves = get_list_of_elves();
    println!("Max: {}", match elves.len() {
        0 => -1,
        n => elves[n-1]
    });
}


fn part_2() {
    let elves = get_list_of_elves();
    println!("Top three: {}", match elves.len() {
        0..=3 => -1,
        n => elves[n-1] + elves[n-2] + elves[n-3]
    });

}

pub fn solve() {
    println!("Day 01");
    part_1();
    part_2();
    println!();
}
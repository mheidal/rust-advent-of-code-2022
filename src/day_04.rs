use crate::read_input;

#[derive(Clone, Copy)]
struct Elf {
    lo: i32,
    hi: i32,
}

fn elf_string_to_elf(elf_string: &str) -> Elf {
    let mut nums = elf_string.split("-");
    let lo = nums.next().expect("Lo value").parse::<i32>().expect("Number");
    let hi = nums.next().expect("Hi value").parse::<i32>().expect("Number");
    Elf { lo, hi }
}

fn compare_both_ways(elf_1: Elf, elf_2: Elf, comparison: fn(Elf, Elf) -> bool) -> bool {
    comparison(elf_1, elf_2) || comparison(elf_2, elf_1)
}

fn full_containment(elf_1: Elf, elf_2: Elf) -> bool {
    let comp = |a: Elf, b: Elf| -> bool { a.lo <= b.lo && b.hi <= a.hi };
    compare_both_ways(elf_1, elf_2, comp)
}

fn overlap(elf_1: Elf, elf_2: Elf) -> bool {
    let comp = |a: Elf, b: Elf| -> bool {a.lo <= b.lo && b.lo <= a.hi};
    compare_both_ways(elf_1, elf_2, comp)
}

fn get_count(comparison_func: fn(Elf, Elf) -> bool) -> i32 {
    let mut count = 0;
    let input = read_input::read("inputs/04.txt");
    for line in input.lines() {
        let mut elves = line.trim().split(",");
        let elf_1 = elf_string_to_elf(elves.next().expect("Elf 1"));
        let elf_2 = elf_string_to_elf(elves.next().expect("Elf 2"));
        if comparison_func(elf_1, elf_2) {
            count += 1;
        }
    }
    count
}

fn part_1() {
    println!("Part 1: {}", get_count(full_containment));
}

fn part_2() {
    println!("Part 2: {}", get_count(overlap));
}

pub fn solve() {
    println!("Day 04");
    part_1();
    part_2();
    println!();
}

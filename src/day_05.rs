use regex::Regex;
use crate::read_input;

struct Move {
    count: usize,
    origin: usize,
    destination: usize,
}

fn one_at_a_time(stacks: &mut [Vec<char>; 9], move_: Move) {
    for _ in 0..move_.count {
        let value = stacks[move_.origin]
            .pop()
            .expect("Top value from origin stack");
        stacks[move_.destination]
            .push(value);
    }
}

fn all_at_once(stacks: &mut [Vec<char>; 9], move_: Move) {
    let tail = stacks[move_.origin]
        .split_off(stacks[move_.origin].len() - move_.count);
    stacks[move_.destination]
        .extend(tail);
}

fn do_moves(modification_method: fn (&mut [Vec<char>; 9], Move) -> ()) -> String {
    const EMPTY_STACK: Vec<char> = Vec::new();
    let mut stacks: [Vec<char>; 9] = [EMPTY_STACK; 9];
    let input = read_input::read("inputs/05.txt");
    let mut lines = input.split("\n");
    let mut in_crates_section = true;
    while in_crates_section {
        let row = lines.next().expect("Line with cargo or stack indices");
        if row.contains('1') {
            in_crates_section = false;
        } else {
            for (i, ch) in row.chars().enumerate() {
                if (&i % 4 == 1) && (ch.is_alphabetic()) {
                    let stack = stacks.get_mut(&i / 4).expect("Stack");
                    stack.push(ch);
                }
            }
        }
    }
    lines.next().expect("Blank line");
    let re = Regex::new(r"\d+").unwrap();
    let moves: Vec<Move> = lines
        .map(|s| {
            let numbers: Vec<usize> = re.find_iter(s)
                .map(|m| m.as_str().parse().unwrap())
                .collect();
            match numbers.as_slice() {
                [count, origin, destination] => Move {
                    count: *count,
                    origin: *origin - 1,
                    destination: *destination - 1
                },
                _ => panic!("Expected three values in vector"),
            }
        })
        .collect();
    for stack in stacks.iter_mut() {
        stack.reverse();
    };

    for move_ in moves {
        modification_method(&mut stacks, move_);
    }
    let mut string = String::new();
    for mut stack in stacks {
        string.push(stack.pop().expect("Some final value"))
    }
    string
}

fn part_1() {
    println!("Part 1: {}", do_moves(one_at_a_time));
}

fn part_2() {
    println!("Part 2: {}", do_moves(all_at_once));
}

pub fn solve() {
    println!("Day 05");
    part_1();
    part_2();
    println!();
}

use crate::day_02::Shape::{Paper, Rock, Scissors};
use crate::read_input;

#[derive(Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

fn parse_letter(letter: &str) -> Shape {
    match letter {
        "A" => Rock,
        "B" => Paper,
        "C" => Scissors,
        "X" => Rock,
        "Y" => Paper,
        "Z" => Scissors,
        _ => panic!("Invalid letter: {letter}"),
    }
}

fn letters_to_shapes(them_letter: &str, you_letter: &str) -> (Shape, Shape) {
    (parse_letter(them_letter), parse_letter(you_letter))
}

fn letters_to_shape_and_condition(them_letter: &str, you_letter: &str) -> (Shape, Shape) {
    let them_shape = parse_letter(them_letter);
    let you_shape = match you_letter {
        "X" => match them_shape {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper,
        },
        "Y" => them_shape,
        "Z" => match them_shape {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock,
        },
        _ => panic!("Invalid letter: {you_letter}"),
    };
    (them_shape, you_shape)
}

fn get_score_from_match(them: Shape, you: Shape) -> i32 {
    match them {
        Rock => {
            match you {
                Rock => 3,
                Paper => 6,
                Scissors => 0,
            }
        },
        Paper => {
            match you {
                Rock => 0,
                Paper => 3,
                Scissors => 6,
            }
        },
        Scissors => {
            match you {
                Rock => 6,
                Paper => 0,
                Scissors => 3,
            }
        },
    }
}

fn get_score_from_strategy(strategy: fn(&str, &str) -> (Shape, Shape)) -> i32 {
    let mut score = 0;
    let input = read_input::read("inputs/02.txt");
    for matchup in input.lines() {
        let mut line = matchup.trim().split(" ");
        let them = line.next()
            .expect("Letter")
            .trim();
        let you = line.next()
            .expect("Letter")
            .trim();
        let (them_shape, you_shape) = strategy(them, you);
        score += get_score_from_match(them_shape, you_shape);
        score += match you_shape {
            Rock => 1,
            Paper => 2,
            Scissors => 3
        }
    }
    score
}

fn part_1() {
    let score= get_score_from_strategy(letters_to_shapes);
    println!("Part 1: Score is {score}");
}


fn part_2() {
    let score= get_score_from_strategy(letters_to_shape_and_condition);
    println!("Part 2: Score is {score}");

}

pub fn solve() {
    println!("Day 02");
    part_1();
    part_2();
    println!();
}
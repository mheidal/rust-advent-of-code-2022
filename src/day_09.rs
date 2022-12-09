use std::collections::HashSet;
use crate::read_input;


#[derive(Clone, Eq, Hash, PartialEq)]
struct PointMotion {
    x: i32,
    y: i32,
}

impl PointMotion {
    fn add(&self, other: PointMotion) -> PointMotion {
        PointMotion {x: self.x + other.x, y: self.y + other.y}
    }
}

fn get_required_motion(leader: &PointMotion, follower: &PointMotion) -> PointMotion {
    let x_diff = follower.x - leader.x;
    let y_diff = follower.y - leader.y;
    if x_diff == 0 {
        if y_diff < -1 {  // follow up
            return PointMotion { x: 0, y: 1 };
        } else if y_diff > 1 {  // follow down
            return PointMotion { x: 0, y: -1 };
        }
    } else if y_diff == 0 {
        if x_diff < -1 {  // follow right
            return PointMotion { x: 1, y: 0 };
        } else if x_diff > 1 {  // follow left
            return PointMotion { x: -1, y: 0 };
        }
    } else if x_diff != 0 && y_diff != 0 && (x_diff.abs() > 1 || y_diff.abs() > 1) {
        if x_diff < 0 {  // L
            if y_diff < 0 {  // LU
                return PointMotion { x: 1, y: 1 };
            } else if y_diff > 0 {  // LD
                return PointMotion { x: 1, y: -1 };
            }
        } else if x_diff > 0 {  // R
            if y_diff < 0 {  // RU
                return PointMotion { x: -1, y: 1 };
            } else if y_diff > 0 {  // RD
                return PointMotion { x: -1, y: -1 };
            }
        }
    }

    PointMotion { x: 0, y: 0 }
}

fn get_knot_histories(num_knots: usize) -> Vec<HashSet<PointMotion>> {
    let mut knots = vec![PointMotion{x: 0, y: 0}; num_knots];
    let mut knot_histories = vec![HashSet::<PointMotion>::new(); num_knots];
    for (i, knot) in knots.iter().enumerate() {
        let _ = knot_histories.get_mut(i).expect("History").insert(knot.clone());
    }
    let input = read_input::read("inputs/09.txt");
    for line in input.lines() {
        let head_motion = match line.trim().split(" ").into_iter().collect::<Vec<&str>>()[..] {
            [letter, ct] => {
                let count = ct.parse::<usize>().expect("Number to iterate");
                match letter {
                    "U" => vec![PointMotion { x: 0, y: 1 }; count],
                    "D" => vec![PointMotion { x: 0, y: -1 }; count],
                    "R" => vec![PointMotion { x: 1, y: 0 }; count],
                    "L" => vec![PointMotion { x: -1, y: 0 }; count],
                    _ => panic!("Unexpected direction")
                }
            },
            _ => panic!("Incorrectly formatted line")
        };
        for head_motion in head_motion {
            knots[0] = knots[0].add(head_motion);
            for i in 0..num_knots-1 {
                let follower_motion = get_required_motion(&knots[i], &knots[i+1]);
                knots[i+1] = knots[i+1].add(follower_motion);
                let _ = knot_histories.get_mut(i + 1).expect("Knot history").insert(knots[i + 1].clone());
            }
        }
    }
    knot_histories
}

fn part_1() {
    let knot_histories = get_knot_histories(2);
    println!("Part 1: {}", knot_histories.get(1).expect("Tail history").len());
}

fn part_2() {
    let knot_histories = get_knot_histories(10);
    println!("Part 1: {}", knot_histories.get(9).expect("Tail history").len());
}

pub fn solve() {
    println!("Day 09");
    part_1();
    part_2();
    println!();
}

use std::collections::HashSet;
use std::{cmp, fmt};
use std::fmt::{Debug, Formatter};
use itertools::Itertools;
use regex::Regex;
use crate::read_input;

#[derive(Clone, Eq, Hash, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Point {
        Point {x, y}
    }
}

#[derive(Clone, Copy)]
struct Range {
    start: i64,
    end: i64,
}

impl Range {
    fn new(start: i64, end: i64) -> Range {
        Range {start, end}
    }

    fn merge(&self, other: Range) -> Range {
        Range::new(
            cmp::min(self.start, other.start),
            cmp::max(self.end, other.end),
        )
    }
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct Line {
    m: i64,
    b: i64,
}

impl Line {
    fn new(m: i64, b: i64) -> Line {
        Line {m, b}
    }
}

fn manhattan(a: &Point, b: &Point) -> i64 {
    (a.x.abs_diff(b.x) + a.y.abs_diff(b.y)) as i64
}

fn ranges_overlap(a: Range, b: Range) -> bool {
    !(a.start > b.end || a.end < b.start)
}

fn resolve_range_overlaps(mut ranges: Vec<Range>) -> Vec<Range> {
    let mut new_ranges: Vec<Range> = vec![];
    ranges.sort_by(|a, b|a.start.cmp(&b.start));
    let mut ranges_iter = ranges.iter();
    let mut cur: Range = *ranges_iter.next().unwrap();
    let mut has_next = true;
    while has_next {
        match ranges_iter.next() {
            Some(r) => {
                let mut next = *r;
                while ranges_overlap(cur, next) {
                    cur = cur.merge(next);
                    match ranges_iter.next() {
                        Some(other) => {
                            next = *other
                        },
                        None => break,
                    };
                }
                new_ranges.push(cur);
                cur = next;
            },
            None => has_next = false,
        }
    }
    new_ranges
}

fn get_scanner_beacons() -> HashSet<(Point, Point)> {
    let input = read_input::read("inputs/15.txt");
    let mut scanner_beacons: HashSet<(Point, Point)> = HashSet::new();
    let re = Regex::new(r"(-?\d+)").unwrap();
    for line in input.lines() {
        let mut nums: Vec<i64> = vec![];
        for cap in re.captures_iter(line) {
            nums.push(cap[1].parse::<i64>().unwrap());
        }
        let scanner = Point {x: nums[0], y: nums[1]};
        let beacon = Point {x: nums[2], y: nums[3]};
        scanner_beacons.insert((scanner, beacon));
    }
    scanner_beacons
}

fn get_ranges(target_row: i64) -> Vec<Range> {
    let mut ranges: Vec<Range> = vec![];
    for (scanner, beacon) in get_scanner_beacons() {
        let manhattan_reach = manhattan(&scanner, &beacon);
        let dist_to_row = target_row.abs_diff(scanner.y) as i64;
        if manhattan_reach >= dist_to_row {
            let flex_along_row = manhattan_reach - dist_to_row;
            let this_range = Range::new(
                scanner.x - flex_along_row,
                scanner.x + flex_along_row
            );
            ranges.push(this_range);
        }
    }
    resolve_range_overlaps(ranges)
}

fn get_line_intersection(a: &Line, b: &Line) -> Point {
    let x = (a.b - b.b) / (b.m - a.m);
    let y = (a.m * x) + a.b;
    Point::new(x, y)
}

fn part_1() {
    let target_row: i64 = 2_000_000;
    let beacons = get_scanner_beacons()
        .iter()
        .map(|sb| sb.1.clone())
        .unique()
        .collect::<Vec<Point>>();
    let ranges = get_ranges(target_row);
    let mut count: i64 = 0;
    for range in ranges{
        count += range.end + 1 - range.start
    }
    count -= beacons
        .iter()
        .filter(|b|b.y == target_row)
        .collect::<Vec<&Point>>()
        .len() as i64;

    println!("Part 1: {}", count);
}

fn part_2() {
    let mut lines: HashSet<Line> = HashSet::new();
    for (scanner, beacon) in get_scanner_beacons() {
        let dist = manhattan(&scanner, &beacon);
        let points = [
            Point::new(scanner.x-dist, scanner.y),
            Point::new(scanner.x+dist, scanner.y),
            Point::new(scanner.x, scanner.y-dist),
            Point::new(scanner.x, scanner.y+dist),
        ];
        for (i, point_1) in points[..].iter().enumerate() {
            for point_2 in points[i+1..].iter() {
                if point_1.x != point_2.x && point_1.y != point_2.y {
                    // this doesn't feel like it should work! It does, but there's probably some
                    // problematic truncation here! fix me!
                    let m = (point_2.y - point_1.y) / (point_2.x - point_1.x);
                    let b = point_1.y - (m * point_1.x);
                    lines.insert(Line::new(m, b));
                }
            }
        }
    }

    let mut line_pairs: Vec<(&Line, &Line)> = vec![];
    let lines: Vec<Line> = lines
        .into_iter()
        .collect::<Vec<Line>>();
    for (i, line_1) in lines[..].iter().enumerate() {
        for line_2 in lines[i+1..].iter() {
            if (line_1.b - line_2.b).abs() == 2 {
                line_pairs.push((
                    cmp::min_by(line_1, line_2, |first, second| first.b.cmp(&second.b)),
                    cmp::max_by(line_1, line_2, |first, second| first.b.cmp(&second.b)),
                ))
            }
        }
    }
    let pair_1 = line_pairs.pop().unwrap();
    let pair_2 = line_pairs.pop().unwrap();
    let intersection = get_line_intersection(pair_1.1, pair_2.0);
    let target_frequency = intersection.x * 4_000_000 + intersection.y;
    println!("Part 2: {}", target_frequency);
}

pub fn solve() {
    println!("Day 15");
    part_1();
    part_2();
    println!();
}

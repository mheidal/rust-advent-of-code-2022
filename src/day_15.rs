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

impl Debug for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
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

impl fmt::Display for Range {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[{},{}]", self.start, self.end)
    }
}

impl fmt::Debug for Range {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[{},{}]", self.start, self.end)
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

impl fmt::Debug for Line {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "y={}x+{}", self.m, self.b)
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
    // println!("Unresolved ranges: {:?}", ranges);
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
    // println!("Resolved ranges: {:?}", new_ranges);
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

fn part_2_brute_force() {
    let scanner_beacons = get_scanner_beacons();
    let mut target_beacon: Option<Point> = None;
    for target_row in 0..=4_000_000 {
        if target_row % 10_000 == 0 {
            println!("{}", target_row);
        }
        match target_beacon {
            Some(_) => break,
            None => (),
        };
        let ranges = get_ranges(target_row);
        for i in 0..ranges.len() - 1 {
            if ranges.get(i+1).unwrap().start - ranges.get(i).unwrap().end == 2 {
                let point_between = Point {x: ranges.get(i).unwrap().end + 1, y: target_row};
                let mut within_any = false;
                for (scanner, beacon) in &scanner_beacons {
                    if manhattan(scanner, beacon) >= manhattan(scanner, &point_between) {
                        within_any = true;
                    }
                }
                if !within_any {
                    target_beacon = Some(point_between);
                }
            }
        }
    }
    let target_beacon = target_beacon.unwrap();
    let target_frequency = target_beacon.x * 4_000_000 + target_beacon.y;
    println!("Part 2: {}", target_frequency);
}

fn part_2_elegant() {
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
    // part_2_brute_force();
    part_2_elegant();
    println!();
}

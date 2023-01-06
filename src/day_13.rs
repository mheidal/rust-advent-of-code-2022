use std::cmp::{min, Ordering};
use crate::read_input;

enum Packet {
    List(Vec<Box<Packet>>),
    Int(i32),
}

impl PartialEq<Self> for Packet {
    fn eq(&self, _other: &Self) -> bool {
        todo!()
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, _other: &Self) -> Option<Ordering> {
        todo!()
    }

    fn lt(&self, other: &Self) -> bool {
        match self {
            Packet::Int(s) => {
                match other {
                    Packet::Int(o) => {
                        *s < *o
                    },
                    Packet::List(_) => {
                        Packet::List(vec![Box::new(Packet::Int(*s))]) < *other
                    },
                }
            },
            Packet::List(s) => {
                match other {
                    Packet::Int(o) => {
                        *self < Packet::List(vec![Box::new(Packet::Int(*o))])
                    },
                    Packet::List(o) => {
                        let mut less: Option<bool> = None;
                        let min_len = min(o.len(), s.len());
                        for i in 0..min_len {
                            let si = s.get(i).unwrap();
                            let oi = o.get(i).unwrap();
                            if si < oi {
                                less = Some(true);
                                break
                            } else if oi < si {
                                less = Some(false);
                                break
                            }
                        }
                        match less {
                            Some(sub_comparison) => sub_comparison,
                            None => s.len() < o.len(),
                        }
                    },
                }
            }
        }
    }
}

fn split_on_outer_commas(s: &str) -> Vec<String> {
    let mut segments: Vec<String> = vec![];
    let mut segment = String::new();
    let mut depth = 0;
    for ch in s.chars() {
        if ch == ',' && depth == 0 {
            segments.push(segment.clone());
            segment = String::new();
        } else {
            if ch == '[' {
                depth += 1;
            } else if ch == ']' {
                depth -= 1;
            }
            segment.push(ch);
        }
    }
    segments.push(segment);
    segments
}

fn parse_packet(packet_str: &str) -> Packet {
    if packet_str == "" {
        Packet::List(vec![])
    } else if packet_str.chars().nth(0).unwrap() == '[' {
        let mut packet = vec![];
        let packet_contents = packet_str
            .strip_prefix('[')
            .unwrap()
            .strip_suffix(']')
            .unwrap();
        for sub_packet_str in split_on_outer_commas(packet_contents).iter().map(|s| s.as_str()) {
            let sub_packet = parse_packet(sub_packet_str);
            packet.push(Box::new(sub_packet));
        }
        Packet::List(packet)
    } else {
        Packet::Int(packet_str.trim().parse::<i32>().unwrap())
    }

}

fn part_1() {
    let input = read_input::read("inputs/13.txt");
    let mut sum = 0;
    for (i, pair) in input.split("\r\n\r\n").enumerate() {
        let mut p = pair.split("\r\n");
        let p1 = parse_packet(p.next().unwrap().trim());
        let p2 = parse_packet(p.next().unwrap().trim());
        if p1 < p2 {
            sum += i + 1;
        } else {
        }
    }
    println!("Part 1: {}", sum);
}

fn part_2() {
    let input = read_input::read("inputs/13.txt");
    let mut packets: Vec<Packet> = vec![];
    for pair in input.split("\r\n\r\n") {
        let mut p = pair.split("\r\n");
        packets.push(parse_packet(p.next().unwrap().trim()));
        packets.push(parse_packet(p.next().unwrap().trim()));
    }
    let packet_two = parse_packet("[[2]]");
    let packet_six = parse_packet("[[6]]");
    let mut two_index = 1; // packet [[2]] starts at index 1 (assume it's lowest)
    let mut six_index = 2; //packet [[6]] starts at index 2 (assume it's lowest except [[2]])
    for p in packets {
        // if a packet is less than either of the two dividers then their true index increments
        // this avoids having to sort the whole list
        if p < packet_two {
            two_index += 1;
            six_index += 1;
        } else if p < packet_six {
            six_index += 1;
        }
    }
    println!("Part 2: {}", six_index * two_index);
}

pub fn solve() {
    println!("Day 13");
    part_1();
    part_2();
    println!();
}

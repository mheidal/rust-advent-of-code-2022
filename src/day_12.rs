use std::borrow::Borrow;
use std::collections::HashMap;
use std::fmt;
use crate::read_input;

#[derive(Clone, Copy, Eq, Hash, PartialEq, Debug)]
struct Coord {
    x: usize,
    y: usize,
}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

struct Node {
    height: i32,
    neighbors: Vec<Coord>,
    visited: bool,
    distance: i32,
}

fn dijkstra(mut nodes: HashMap<Coord, Node>, start: Coord, targets: Vec<Coord>) -> i32 {
    let mut current: Coord;
    nodes.get_mut(&start).unwrap().distance = 0;
    let mut any_target_reached: bool = false;
    let mut all_nodes_visited: bool = false;
    while !any_target_reached && !all_nodes_visited {
        current = *nodes
            .borrow()
            .iter()
            .filter(|(_c, n)| !n.visited && n.distance != 1_000_000)
            .min_by(|a, b| a.1.distance.cmp(&b.1.distance))
            .unwrap()
            .0;
        nodes.get_mut(&current).unwrap().visited = true;
        let neighbor_coords = nodes.get(&current).unwrap().neighbors.clone();
        let current_distance = nodes.get(&current).unwrap().distance;
        for neighbor_coord in neighbor_coords {
            let mut neighbor = nodes.get_mut(&neighbor_coord).unwrap();
            if neighbor.visited {
                continue
            }
            neighbor.distance = if neighbor.distance > current_distance + 1 {
                current_distance + 1
            } else {
                neighbor.distance
            };
        }
        for target in &targets {
            if nodes.get(target).unwrap().visited {
                any_target_reached = true;
            }
        }
        all_nodes_visited = nodes
            .borrow()
            .iter()
            .filter(|(_c, n)| !n.visited)
            .collect::<Vec<(&Coord, &Node)>>()
            .len() == 0;
    }
    nodes.get(targets
        .iter()
        .filter(|c| nodes.get(c).unwrap().visited)
        .next()
        .unwrap())
        .unwrap()
        .distance
}

fn do_search(lo_to_hi: bool) -> i32 {

    let mut nodes: HashMap<Coord, Node> = HashMap::new();
    let mut start: Option<Coord> = None;
    let mut targets: Vec<Coord> = Vec::new();
    let input = read_input::read("inputs/12.txt");
    let mut max_x = 0;
    let mut max_y = 0;
    for (i, line) in input.lines().enumerate() {
        if i > max_y {
            max_y = i;
        }
        for (j, ch) in line.trim().chars().enumerate() {
            if j > max_x {
                max_x = j;
            }
            let coord = Coord { x: j, y: i };
            let node = Node {
                height: match ch {
                    'S' => {
                        if lo_to_hi {
                            match start {
                                Some(_) => (),
                                None => start = Some(coord)
                            }
                        } else {
                            targets.push(coord)
                        }
                        'a' as i32
                    },
                    'E' => {
                        if lo_to_hi {
                            targets.push(coord);
                        } else {
                            match start {
                                Some(_) => (),
                                None => start = Some(coord)
                            }
                        }
                        'z' as i32
                    },
                    h => {
                        if h == 'a' && !lo_to_hi {
                            targets.push(coord)
                        }
                        h as i32
                    }
                },
                neighbors: Vec::new(),
                visited: false,
                distance: 1_000_000,
            };
            nodes.insert(coord, node);
        }
    }
    let mut edges: HashMap<Coord, Vec<Coord>> = HashMap::new();
    for (coord, node) in &*&nodes {
        let mut possible_neighbors: Vec<Coord> = vec![];

        if coord.x > 0 {
            possible_neighbors.push(Coord { x: coord.x - 1, y: coord.y });
        }
        if coord.x < max_x {
            possible_neighbors.push(Coord { x: coord.x + 1, y: coord.y })
        }
        if coord.y > 0 {
            possible_neighbors.push(Coord { x: coord.x, y: coord.y - 1 });
        }
        if coord.y < max_y  {
            possible_neighbors.push(Coord { x: coord.x, y: coord.y + 1 })
        }
        for neighbor in possible_neighbors {
            match nodes.get(&neighbor) {
                Some(other_node) => {
                    if lo_to_hi {
                        if other_node.height - node.height <= 1 {
                            edges
                                .entry(*coord)
                                .or_insert(vec![])
                                .push(neighbor);
                        } else {
                        }
                    } else {
                        if node.height - other_node.height <= 1 {
                            edges
                                .entry(*coord)
                                .or_insert(vec![])
                                .push(neighbor);
                        }
                    }
                },
                None => ()
            };
        }
    }
    for (coord, neighbors) in edges.iter() {
        nodes.get_mut(coord).unwrap().neighbors.extend(neighbors);
    }
    dijkstra(nodes, start.unwrap(), targets)
}

fn part_1() {
    println!("Part 1: {}", do_search(true));
}

fn part_2() {
    println!("Part 2: {}", do_search(false));
}

pub fn solve() {
    println!("Day 12");
    part_1();
    part_2();
    println!();
}

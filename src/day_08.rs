use crate::read_input;

fn build_grid() -> Vec<Vec<i32>> {
    let mut grid: Vec<Vec<i32>> = Vec::<Vec<i32>>::new();
    let input = read_input::read("inputs/08.txt");
    for line in input.lines() {
        let mut row: Vec<i32> = vec![];
        for character in line.chars() {
            row.push(character.to_digit(10).expect("Digit") as i32);
        }
        grid.push(row);
    }
    grid
}

fn get_col(grid: &Vec<Vec<i32>>, col_index: usize) -> Vec<i32> {
    let mut col: Vec<i32> = vec![];
    for row in grid {
        col.push(row[col_index]);
    }
    col
}

fn get_vectors_of_directions(row: &Vec<i32>, col_index: usize, col: &Vec<i32>, row_index: usize) -> [Vec<i32>; 4] {
    [
        row.clone()[..col_index].iter().map(|i|*i).rev().collect::<Vec<i32>>(),
        row.clone()[col_index +1..].iter().map(|i|*i).collect::<Vec<i32>>(),
        col.clone()[..row_index].iter().map(|i|*i).rev().collect::<Vec<i32>>(),
        col.clone()[row_index +1..].iter().map(|i|*i).collect::<Vec<i32>>(),
    ]
}

fn vis_from_direction(trees_in_direction: Vec<i32>, orig_tree: &i32) -> bool {
    match trees_in_direction.into_iter().max() {
        None => true,
        Some(other_tree) => other_tree < *orig_tree,
    }
}

fn is_visible(row_index: usize, col_index: usize, grid: &Vec<Vec<i32>>) -> bool {
    let tree = grid[row_index][col_index];
    let col = get_col(&grid, col_index);
    let direction_vectors = get_vectors_of_directions(&grid[row_index], col_index, &col, row_index);
    let mut visibilities: Vec<bool> = vec![];
    for direction_vector in direction_vectors {
        visibilities.push(vis_from_direction(direction_vector, &tree))
    }
    visibilities.contains(&true)
}

fn scenic_score_factor(trees_in_direction: Vec<i32>, orig_tree: &i32) -> i32 {
    let mut factor = 0;
    for tree in trees_in_direction {
        factor += 1;
        if tree >= *orig_tree {
            break
        }
    }
    factor
}

fn get_scenic_score(row_index: usize, col_index: usize, grid: &Vec<Vec<i32>>) -> i32 {
    let col = get_col(&grid, col_index);
    let direction_vectors = get_vectors_of_directions(&grid[row_index], col_index, &col, row_index);
    let mut score = 1;
    let tree = grid[row_index][col_index];
    for direction_vector in direction_vectors {
        score *= scenic_score_factor(direction_vector, &tree)
    }
    score
}

fn part_1() {
    let mut count = 0;
    let grid = build_grid();
    let glen: usize = grid.len();
    for row_index in 0..glen {
        for col_index in 0..glen {
            if is_visible(row_index, col_index, &grid) {
                count += 1
            }
        }
    }
    println!("Part 1: {}", count);
}

fn part_2() {
    let mut scores: Vec<i32> = vec![];
    let grid = build_grid();
    let glen: usize = grid.len();
    for row_index in 0..glen {
        for col_index in 0..glen {
            scores.push(get_scenic_score(row_index, col_index, &grid))
        }
    }
    println!("Part 2: {}", scores.iter().max().expect("At least one scenic score"));
}

pub fn solve() {
    println!("Day 08");
    part_1();
    part_2();
    println!();
}
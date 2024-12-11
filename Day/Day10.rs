use std::collections::{HashSet, VecDeque};
use std::fs;

/// Parse the input into a 2D vector of integers
fn parse_map(input: &str) -> Vec<Vec<u8>> {
    input.lines()
        .map(|line| line.chars().map(|ch| ch.to_digit(10).unwrap() as u8).collect())
        .collect()
}

/// Find all valid neighbors that can be visited from a given position
fn valid_neighbors(map: &Vec<Vec<u8>>, x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let current_height = map[x][y];

    for &(dx, dy) in &directions {
        let nx = x as isize + dx;
        let ny = y as isize + dy;

        if nx >= 0 && ny >= 0 {
            let nx = nx as usize;
            let ny = ny as usize;

            if nx < map.len() && ny < map[0].len() && map[nx][ny] == current_height + 1 {
                neighbors.push((nx, ny));
            }
        }
    }
    neighbors
}

/// Calculate the score for a given trailhead using BFS
fn calculate_score(map: &Vec<Vec<u8>>, start_x: usize, start_y: usize) -> usize {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut reachable_nines = HashSet::new();

    queue.push_back((start_x, start_y));
    visited.insert((start_x, start_y));

    while let Some((x, y)) = queue.pop_front() {
        for (nx, ny) in valid_neighbors(map, x, y) {
            if !visited.contains(&(nx, ny)) {
                visited.insert((nx, ny));
                queue.push_back((nx, ny));

                // Record reachable height 9
                if map[nx][ny] == 9 {
                    reachable_nines.insert((nx, ny));
                }
            }
        }
    }

    reachable_nines.len()
}

/// Main function to calculate the total score of all trailheads
fn sum_trailhead_scores(file_path: &str) -> usize {
    let input = fs::read_to_string(file_path)
        .expect("Failed to read input file");
    let map = parse_map(&input);
    let mut total_score = 0;

    for x in 0..map.len() {
        for y in 0..map[0].len() {
            if map[x][y] == 0 {
                total_score += calculate_score(&map, x, y);
            }
        }
    }

    total_score
}

fn main() {
    let file_path = "Day7.txt"; // Replace with the path to your input file
    let total_score = sum_trailhead_scores(file_path);
    println!("The total score of all trailheads is: {}", total_score);
}

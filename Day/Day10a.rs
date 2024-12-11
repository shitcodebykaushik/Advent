use std::collections::HashMap;
use std::fs;

/// Parse the input into a 2D vector of integers
fn parse_map(input: &str) -> Vec<Vec<u8>> {
    input.lines()
        .map(|line| line.chars().map(|ch| ch.to_digit(10).unwrap() as u8).collect())
        .collect()
}

/// Find all valid neighbors for moving uphill
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

/// Recursive DP function to count paths to height 9
fn count_paths(
    map: &Vec<Vec<u8>>,
    x: usize,
    y: usize,
    memo: &mut HashMap<(usize, usize), usize>,
) -> usize {
    // If this cell is already computed, return the cached value
    if let Some(&result) = memo.get(&(x, y)) {
        return result;
    }

    // If this cell is height 9, there is exactly one path ending here
    if map[x][y] == 9 {
        return 1;
    }

    // Otherwise, calculate the number of paths recursively
    let mut path_count = 0;
    for (nx, ny) in valid_neighbors(map, x, y) {
        path_count += count_paths(map, nx, ny, memo);
    }

    // Cache the result for this cell
    memo.insert((x, y), path_count);
    path_count
}

/// Calculate the rating for a given trailhead
fn calculate_rating(map: &Vec<Vec<u8>>, start_x: usize, start_y: usize) -> usize {
    let mut memo = HashMap::new();
    count_paths(map, start_x, start_y, &mut memo)
}

/// Main function to calculate the total rating of all trailheads
fn sum_trailhead_ratings(file_path: &str) -> usize {
    let input = fs::read_to_string(file_path)
        .expect("Failed to read input file");
    let map = parse_map(&input);
    let mut total_rating = 0;

    for x in 0..map.len() {
        for y in 0..map[0].len() {
            if map[x][y] == 0 {
                total_rating += calculate_rating(&map, x, y);
            }
        }
    }

    total_rating
}

fn main() {
    let file_path = "Day7.txt"; // Replace with the path to your input file
    let total_rating = sum_trailhead_ratings(file_path);
    println!("The total rating of all trailheads is: {}", total_rating);
}

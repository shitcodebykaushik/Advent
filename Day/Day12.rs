use std::fs;
use std::collections::{HashMap, HashSet, VecDeque};

type Point = (usize, usize);

fn calculate_price(map: Vec<Vec<char>>) -> usize {
    let rows = map.len();
    let cols = map[0].len();
    let mut visited = vec![vec![false; cols]; rows];
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    
    let mut total_price = 0;

    for r in 0..rows {
        for c in 0..cols {
            if !visited[r][c] {
                let plant = map[r][c];
                let mut area = 0;
                let mut perimeter = 0;

                // Perform BFS to explore the region
                let mut queue = VecDeque::new();
                queue.push_back((r, c));
                visited[r][c] = true;

                while let Some((x, y)) = queue.pop_front() {
                    area += 1;

                    // Check all 4 directions
                    for &(dx, dy) in &directions {
                        let nx = x as isize + dx;
                        let ny = y as isize + dy;

                        if nx >= 0 && ny >= 0 && (nx as usize) < rows && (ny as usize) < cols {
                            let (nx, ny) = (nx as usize, ny as usize);
                            if map[nx][ny] == plant {
                                if !visited[nx][ny] {
                                    visited[nx][ny] = true;
                                    queue.push_back((nx, ny));
                                }
                            } else {
                                perimeter += 1; // Boundary with a different plant
                            }
                        } else {
                            perimeter += 1; // Boundary with the edge of the map
                        }
                    }
                }

                total_price += area * perimeter;
            }
        }
    }

    total_price
}

fn read_map_from_file(filename: &str) -> Vec<Vec<char>> {
    let contents = fs::read_to_string(filename).expect("Failed to read the file");
    contents
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn main() {
    let filename = "Day7.txt";
    let map = read_map_from_file(filename);
    let total_price = calculate_price(map);
    println!("Total price of fencing all regions: {}", total_price);
}

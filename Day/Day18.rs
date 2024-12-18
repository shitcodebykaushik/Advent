use std::collections::{HashSet, BinaryHeap};
use std::cmp::Ordering;
use std::fs;

const GRID_SIZE: usize = 71; // Memory space dimensions (0-70 inclusive)
const BYTES_TO_SIMULATE: usize = 1024; // Number of bytes to simulate

type Point = (usize, usize);

#[derive(Eq, PartialEq)]
struct State {
    cost: usize,
    position: Point,
}

// Priority queue ordering for the A* algorithm
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost) // Reverse for min-heap
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Manhattan distance heuristic
fn heuristic(a: Point, b: Point) -> usize {
    (a.0 as isize - b.0 as isize).abs() as usize + (a.1 as isize - b.1 as isize).abs() as usize
}

// Parse the falling bytes into corrupted points
fn parse_bytes(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(|line| {
            let coords: Vec<usize> = line.split(',').map(|n| n.parse().unwrap()).collect();
            (coords[0], coords[1])
        })
        .collect()
}

// Simulate byte corruption on the grid
fn simulate_corruption(bytes: &[Point]) -> HashSet<Point> {
    bytes.iter().take(BYTES_TO_SIMULATE).cloned().collect()
}

// Find the shortest path using A* algorithm
fn find_shortest_path(corrupted: &HashSet<Point>) -> Option<usize> {
    let start = (0, 0);
    let goal = (GRID_SIZE - 1, GRID_SIZE - 1);
    let mut heap = BinaryHeap::new();
    let mut visited = HashSet::new();

    heap.push(State {
        cost: 0,
        position: start,
    });

    while let Some(State { cost, position }) = heap.pop() {
        if position == goal {
            return Some(cost);
        }

        if visited.contains(&position) {
            continue;
        }

        visited.insert(position);

        let (x, y) = position;
        for (dx, dy) in [(0, 1), (1, 0), (0, usize::MAX), (usize::MAX, 0)].iter().copied() {
            let nx = x.wrapping_add(dx);
            let ny = y.wrapping_add(dy);
            if nx < GRID_SIZE && ny < GRID_SIZE {
                let neighbor = (nx, ny);
                if !corrupted.contains(&neighbor) && !visited.contains(&neighbor) {
                    heap.push(State {
                        cost: cost + 1,
                        position: neighbor,
                    });
                }
            }
        }
    }
    None // No path found
}

fn main() {
    // Load input from file
    let filename = "Day7.txt"; // Replace with the path to your input file
    let input = fs::read_to_string(filename).expect("Failed to read input file");

    let bytes = parse_bytes(&input);
    let corrupted = simulate_corruption(&bytes);

    match find_shortest_path(&corrupted) {
        Some(steps) => println!("Minimum steps to exit: {}", steps),
        None => println!("No path to the exit."),
    }
}

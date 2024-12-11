use std::fs;
use std::collections::VecDeque;

fn split_number(num: u64) -> (u64, u64) {
    let digits = num.to_string();
    let mid = digits.len() / 2;
    let left = digits[..mid].parse::<u64>().unwrap_or(0);
    let right = digits[mid..].parse::<u64>().unwrap_or(0);
    (left, right)
}

fn simulate_blinks(mut stones: Vec<u64>, blinks: usize) -> usize {
    let mut queue: VecDeque<u64> = VecDeque::from(stones);

    for _ in 0..blinks {
        let mut new_queue = VecDeque::new();
        
        while let Some(stone) = queue.pop_front() {
            if stone == 0 {
                new_queue.push_back(1);
            } else if stone.to_string().len() % 2 == 0 {
                let (left, right) = split_number(stone);
                new_queue.push_back(left);
                new_queue.push_back(right);
            } else {
                new_queue.push_back(stone * 2024);
            }
        }
        queue = new_queue;
    }

    queue.len()
}

fn load_stones_from_file(file_path: &str) -> Result<Vec<u64>, std::io::Error> {
    let content = fs::read_to_string(file_path)?;
    let stones = content
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();
    Ok(stones)
}

fn main() {
    let file_path = "Day7.txt"; // Replace with your file name
    let blinks = 25;

    match load_stones_from_file(file_path) {
        Ok(initial_stones) => {
            let result = simulate_blinks(initial_stones, blinks);
            println!("Number of stones after {} blinks: {}", blinks, result);
        }
        Err(err) => {
            eprintln!("Error reading file: {}", err);
        }
    }
}

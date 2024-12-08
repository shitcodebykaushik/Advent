use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn evaluate_left_to_right(nums: &[i64], ops: &[char]) -> i64 {
    let mut result = nums[0];
    for (i, &op) in ops.iter().enumerate() {
        match op {
            '+' => result += nums[i + 1],
            '*' => result *= nums[i + 1],
            _ => unreachable!(),
        }
    }
    result
}

fn generate_all_operations(nums: &[i64], target: i64) -> bool {
    let n = nums.len() - 1; 
    let ops = ['+', '*'];
    let mut valid = false;
    for mask in 0..(1 << n) {
        let mut current_ops = vec![];
        for i in 0..n {
            current_ops.push(ops[(mask >> i) & 1]);
        }
        if evaluate_left_to_right(nums, &current_ops) == target {
            valid = true;
            break;
        }
    }
    valid
}

fn total_calibration_result(input: &str) -> i64 {
    let mut total = 0;

    for line in input.lines() {
        if let Some((target, numbers)) = line.split_once(": ") {
            let target: i64 = target.parse().unwrap();
            let nums: Vec<i64> = numbers.split_whitespace().map(|n| n.parse().unwrap()).collect();

            if generate_all_operations(&nums, target) {
                total += target;
            }
        }
    }

    total
}

fn read_file(file_path: &str) -> io::Result<String> {
    let path = Path::new(file_path);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);
    let mut contents = String::new();
    for line in reader.lines() {
        contents.push_str(&line?);
        contents.push('\n');
    }
    Ok(contents)
}

fn main() {
    let file_path = "Day7.txt"; 
    match read_file(file_path) {
        Ok(input) => {
            let result = total_calibration_result(&input);
            println!("Total calibration result: {}", result);
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
        }
    }
}

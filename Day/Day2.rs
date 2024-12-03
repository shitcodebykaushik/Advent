use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn is_safe_report(report: &[i32]) -> bool {
    if report.len() < 2 {
        return false; 
    }

    let mut is_increasing = true;
    let mut is_decreasing = true;

    for i in 0..report.len() - 1 {
        let diff = report[i + 1] - report[i];

        if diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }

        if diff > 0 {
            is_decreasing = false;
        } else if diff < 0 {
            is_increasing = false;
        }
    }

    is_increasing || is_decreasing
}

fn count_safe_reports<P: AsRef<Path>>(file_path: P) -> io::Result<usize> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut safe_count = 0;

    for line in reader.lines() {
        let line = line?;
        let report: Vec<i32> = line
            .split_whitespace()
            .filter_map(|num| num.parse::<i32>().ok())
            .collect();

        if is_safe_report(&report) {
            safe_count += 1;
        }
    }

    Ok(safe_count)
}

fn main() {
    let file_path = "data2.txt"; 

    match count_safe_reports(file_path) {
        Ok(safe_count) => println!("Number of safe reports: {}", safe_count),
        Err(e) => eprintln!("Error reading the file: {}", e),
    }
}

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/// Check if a single report is safe based on the given rules.
fn is_safe_report(report: &[i32]) -> bool {
    if report.len() < 2 {
        return false; // Reports with less than 2 levels cannot satisfy the rules.
    }

    let mut is_increasing = true;
    let mut is_decreasing = true;

    for i in 0..report.len() - 1 {
        let diff = report[i + 1] - report[i];

        // Check the range of the difference
        if diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }

        // Check for increasing or decreasing trends
        if diff > 0 {
            is_decreasing = false;
        } else if diff < 0 {
            is_increasing = false;
        }
    }

    // Must be strictly increasing or strictly decreasing
    is_increasing || is_decreasing
}

/// Check if a report can be made safe by removing a single level.
fn can_be_safe_with_dampener(report: &[i32]) -> bool {
    if is_safe_report(report) {
        return true; // Already safe without changes.
    }

    for i in 0..report.len() {
        // Create a new report by omitting the i-th level.
        let mut modified_report = Vec::with_capacity(report.len() - 1);
        modified_report.extend_from_slice(&report[..i]);
        modified_report.extend_from_slice(&report[i + 1..]);

        if is_safe_report(&modified_report) {
            return true;
        }
    }

    false
}

/// Count the number of safe reports in the file data.
fn count_safe_reports_with_dampener<P: AsRef<Path>>(file_path: P) -> io::Result<usize> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut safe_count = 0;

    for line in reader.lines() {
        let line = line?;
        let report: Vec<i32> = line
            .split_whitespace()
            .filter_map(|num| num.parse::<i32>().ok())
            .collect();

        if can_be_safe_with_dampener(&report) {
            safe_count += 1;
        }
    }

    Ok(safe_count)
}

fn main() {
    let file_path = "data2a.txt"; // Path to the input file

    match count_safe_reports_with_dampener(file_path) {
        Ok(safe_count) => println!("Number of safe reports: {}", safe_count),
        Err(e) => eprintln!("Error reading the file: {}", e),
    }
}

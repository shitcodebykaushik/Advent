//
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {

    let file_path = "data.txt";

    
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);


    let mut arr1 = Vec::new();
    let mut arr2 = Vec::new();


    for line in reader.lines() {
        let line = line?;
        let mut nums = line.split_whitespace()
            .filter_map(|num| num.parse::<i32>().ok());
        if let (Some(num1), Some(num2)) = (nums.next(), nums.next()) {
            arr1.push(num1);
            arr2.push(num2);
        }
    }

    
    arr1.sort_unstable();
    arr2.sort_unstable();

    let total: i64 = arr1.iter()
        .zip(arr2.iter())
        .map(|(&x, &y)| (x - y).abs() as i64)
        .sum();

    for (x, y) in arr1.iter().zip(arr2.iter()) {
        println!("{} {}", x, y);
    }
    println!("total is {}", total);

    Ok(())
}


// This is the second question of the first day
use std::fs;
use std::error::Error;
use std::collections::HashMap;

fn calculate_similarity_score(left: &[i32], right: &[i32]) -> i32 {
    let mut right_count = HashMap::new();
    for &num in right {
        *right_count.entry(num).or_insert(0) += 1;
    }

    let mut similarity_score = 0;
    for &num in left {
        if let Some(&count) = right_count.get(&num) {
            similarity_score += num * count;
        }
    }

    similarity_score
}

fn main() -> Result<(), Box<dyn Error>> {
    // Read file content
    let data = fs::read_to_string("data.txt")?;
    
    // Split lines and parse columns
    let mut left = Vec::new();
    let mut right = Vec::new();
    for line in data.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            left.push(parts[0].parse::<i32>()?);
            right.push(parts[1].parse::<i32>()?);
        }
    }

    // Compute the similarity score
    let score = calculate_similarity_score(&left, &right);
    println!("Similarity Score: {}", score);

    Ok(())
}

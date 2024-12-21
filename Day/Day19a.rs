use std::collections::HashSet;
use std::fs;

fn count_arrangements(towel_patterns: Vec<&str>, designs: Vec<&str>) -> usize {
    let patterns: HashSet<&str> = towel_patterns.into_iter().collect();
    let mut total_count = 0;

    for design in designs {
        let design_len = design.len();
        let mut dp = vec![0; design_len + 1];
        dp[0] = 1; // Base case: 1 way to construct the empty string.

        for i in 1..=design_len {
            for j in 0..i {
                if patterns.contains(&design[j..i]) {
                    dp[i] += dp[j];
                }
            }
        }

        total_count += dp[design_len]; // Add all ways to form the current design.
    }

    total_count
}

fn main() -> std::io::Result<()> {
    // Load the input from the file
    let input = fs::read_to_string("Day7 .txt")?;
    
    // Split the input into sections
    let mut sections = input.split("\n\n");
    
    // Parse towel patterns
    let towel_patterns: Vec<&str> = sections
        .next()
        .expect("No towel patterns found")
        .split(", ")
        .collect();
    
    // Parse designs
    let designs: Vec<&str> = sections
        .next()
        .expect("No designs found")
        .lines()
        .collect();

    // Calculate the total number of arrangements
    let result = count_arrangements(towel_patterns, designs);
    println!("Total number of arrangements: {}", result);

    Ok(())
}

use std::collections::HashSet;
use std::fs;

fn count_possible_designs(towel_patterns: Vec<&str>, designs: Vec<&str>) -> usize {
    let patterns: HashSet<&str> = towel_patterns.into_iter().collect();
    let mut possible_count = 0;

    for design in designs {
        let design_len = design.len();
        let mut dp = vec![false; design_len + 1];
        dp[0] = true; // Base case: empty design is always possible.

        for i in 1..=design_len {
            for j in 0..i {
                if dp[j] && patterns.contains(&design[j..i]) {
                    dp[i] = true;
                    break;
                }
            }
        }

        if dp[design_len] {
            possible_count += 1;
        }
    }

    possible_count
}

fn main() -> std::io::Result<()> {
    // Load the input from the file
    let input = fs::read_to_string("Day7.txt")?;
    
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

    // Calculate the number of possible designs
    let result = count_possible_designs(towel_patterns, designs);
    println!("Number of possible designs: {}", result);

    Ok(())
}

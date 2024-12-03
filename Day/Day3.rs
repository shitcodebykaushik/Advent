use regex::Regex;
use std::fs;
use std::io;

/// Function to calculate the sum of valid multiplications in the corrupted memory.
fn sum_valid_multiplications(file_path: &str) -> io::Result<i32> {
    // Read the content of the file into a string
    let memory = fs::read_to_string(file_path)?;

    // Define a regex pattern for valid `mul(X, Y)` instructions
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut sum = 0;

    // Iterate over all matches of the regex in the memory string
    for cap in re.captures_iter(&memory) {
        // Extract the two numbers X and Y
        let x: i32 = cap[1].parse().unwrap();
        let y: i32 = cap[2].parse().unwrap();

        // Multiply and add the result to the sum
        sum += x * y;
    }

    Ok(sum)
}

fn main() {
    let file_path = "Data2.txt"; // Path to the input file

    match sum_valid_multiplications(file_path) {
        Ok(result) => println!("Sum of valid multiplications: {}", result),
        Err(e) => eprintln!("Error: {}", e),
    }
}

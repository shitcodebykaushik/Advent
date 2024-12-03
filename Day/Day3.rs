use regex::Regex;
use std::fs;
use std::io;

fn sum_valid_multiplications(file_path: &str) -> io::Result<i32> {
    let memory = fs::read_to_string(file_path)?;

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut sum = 0;

    for cap in re.captures_iter(&memory) {
        let x: i32 = cap[1].parse().unwrap();
        let y: i32 = cap[2].parse().unwrap();

        sum += x * y;
    }

    Ok(sum)
}

fn main() {
    let file_path = "Data2.txt"; 

    match sum_valid_multiplications(file_path) {
        Ok(result) => println!("Sum of valid multiplications: {}", result),
        Err(e) => eprintln!("Error: {}", e),
    }
}

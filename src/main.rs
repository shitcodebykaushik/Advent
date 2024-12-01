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

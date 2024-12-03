use regex::Regex;
use std::fs;

fn main() {
   
    let input = fs::read_to_string("data3a.txt")
        .expect("Failed to read input.txt file")
        .replace('\n', ""); 
   
    let input = format!("do(){}", input);

    
    let segments = input.split("don't()");

    let mut result = 0;


    let mul_regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    for segment in segments {
      
        let do_segments: Vec<&str> = segment.split("do()").collect();

       
        for to_mul in do_segments.iter().skip(1) {
           
            let products: i64 = mul_regex
                .captures_iter(to_mul)
                .map(|caps| {
                    let x: i64 = caps[1].parse().unwrap();
                    let y: i64 = caps[2].parse().unwrap();
                    x * y
                })
                .sum();

            result += products;
        }
    }

    println!("{}", result);
}

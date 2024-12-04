use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        std::process::exit(1);
    }

    let input_file = &args[1];

    let input = fs::read_to_string(input_file)
        .expect("Failed to read input file");

    let lines: Vec<&str> = input.lines().collect();
    let y = lines.len();
    let x = lines.get(0).map_or(0, |line| line.len());

    let grid: Vec<char> = lines.iter().flat_map(|line| line.chars()).collect();

    let mut found = 0;

    for j in 1..y - 1 {
        for i in 1..x - 1 {
            let idx = j * x + i;

            // Check the current character
            if grid[idx] != 'A' {
                continue;
            }

            let c = [
                grid[(j - 1) * x + (i - 1)],
                grid[(j - 1) * x + (i + 1)],
                grid[(j + 1) * x + (i - 1)], 
                grid[(j + 1) * x + (i + 1)], 
            ];

   
            if c[0] == c[3] || c[1] == c[2] {
                continue;
            }

            
            if c.iter().all(|&ch| ch == 'M' || ch == 'S') {
                found += 1;
            }
        }
    }

    println!("{}", found);
}

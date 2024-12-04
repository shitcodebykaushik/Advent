use std::fs;

fn count_word(grid: &[Vec<char>], word: &str) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let word_len = word.len();
    let word_chars: Vec<char> = word.chars().collect();
    let mut count = 0;

    let directions = [
        (0, 1),  
        (0, -1), 
        (1, 0),  
        (-1, 0), 
        (1, 1),  
        (1, -1),  
        (-1, 1),  
        (-1, -1), 
    ];

    for row in 0..rows {
        for col in 0..cols {
            for &(dr, dc) in &directions {
                let mut found = true;
                for i in 0..word_len {
                    let r = row as isize + i as isize * dr;
                    let c = col as isize + i as isize * dc;

                    if r < 0 || r >= rows as isize || c < 0 || c >= cols as isize {
                        found = false;
                        break;
                    }

                    if grid[r as usize][c as usize] != word_chars[i] {
                        found = false;
                        break;
                    }
                }
                if found {
                    count += 1;
                }
            }
        }
    }

    count
}

fn main() {
 
    let input = fs::read_to_string("data4.txt").expect("Failed to read input.txt");
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

   
    let word = "XMAS";

    // Count occurrences of the word
    let occurrences = count_word(&grid, word);

    println!("The word '{}' appears {} times in the grid.", word, occurrences);
}

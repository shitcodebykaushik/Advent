use std::{fs, io};

struct LocksAndKeys {
    locks: Vec<Vec<usize>>,
    keys: Vec<Vec<usize>>,
}

fn lock_heights(block: &[Vec<char>]) -> Vec<usize> {
    let mut heights = vec![0; block[0].len()];

    let all_hashes_row = block
        .iter()
        .position(|row| row.iter().all(|&c| c == '#'))
        .expect("The block should have a row of hashes at the top");
    assert_eq!(all_hashes_row, 0, "Expected the row of hashes to be at the top");

    for row in block.iter().skip(all_hashes_row + 1) {
        for (col, &c) in row.iter().enumerate() {
            if c == '#' {
                heights[col] += 1;
            }
        }
    }

    heights
}

fn key_heights(block: &[Vec<char>]) -> Vec<usize> {
    let mut heights = vec![0; block[0].len()];

    let all_hashes_row = block
        .iter()
        .rposition(|row| row.iter().all(|&c| c == '#'))
        .expect("The block should have a row of hashes at the bottom");
    assert_eq!(all_hashes_row, block.len() - 1, "Expected the row of hashes to be at the bottom");

    for row in block.iter().take(all_hashes_row) {
        for (col, &c) in row.iter().enumerate() {
            if c == '#' {
                heights[col] += 1;
            }
        }
    }

    heights
}

fn lock_fits_key(lock: &[usize], key: &[usize]) -> bool {
    lock.iter()
        .zip(key.iter())
        .all(|(lock_height, key_height)| lock_height + key_height < 6)
}

fn read_input(filepath: &str) -> io::Result<LocksAndKeys> {
    let input = fs::read_to_string(filepath)?;

    let split = input.split("\n\n");

    let mut locks: Vec<Vec<usize>> = Vec::new();
    let mut keys: Vec<Vec<usize>> = Vec::new();

    for block in split {
        let block: Vec<Vec<char>> = block.lines().map(|line| line.chars().collect()).collect();

        let all_hashes_row = block
            .iter()
            .position(|row| row.iter().all(|&c| c == '#'))
            .expect("Every block must contain at least one row of hashes");

        if all_hashes_row == 0 {
            locks.push(lock_heights(&block));
        } else {
            keys.push(key_heights(&block));
        }
    }

    Ok(LocksAndKeys { locks, keys })
}

fn main() -> io::Result<()> {
    let locks_and_keys = read_input("Day7.txt")?;

    let ans_one = locks_and_keys
        .locks
        .iter()
        .flat_map(|lock| {
            locks_and_keys
                .keys
                .iter()
                .filter(move |key| lock_fits_key(lock, key))
        })
        .count();
    println!("Ans Part One: {ans_one}");

    Ok(())
}

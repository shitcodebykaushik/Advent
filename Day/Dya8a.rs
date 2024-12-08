use std::collections::HashSet;

fn main() {
    let input = include_str!("../Day7.txt");
    let grid: Vec<&str> = input.lines().collect();

    let width = grid[0].len();
    let height = grid.len();

    let mut antennas: Vec<Vec<(i32, i32)>> = vec![Vec::new(); 62]; // 62 for '0'-'9', 'A'-'Z', 'a'-'z'

    // Parse input to find antennas
    for (y, line) in grid.iter().enumerate() {
        for (x, &ch) in line.as_bytes().iter().enumerate() {
            if ch != b'.' {
                let idx = if ch.is_ascii_digit() {
                    (ch - b'0') as usize
                } else if ch.is_ascii_uppercase() {
                    10 + (ch - b'A') as usize
                } else {
                    36 + (ch - b'a') as usize
                };
                antennas[idx].push((x as i32, y as i32));
            }
        }
    }

    let mut antinodes = HashSet::new();

    // Process each frequency
    for nodes in antennas.iter().filter(|nodes| !nodes.is_empty()) {
        let n = nodes.len();
        for i in 0..n {
            for j in i + 1..n {
                let (x1, y1) = nodes[i];
                let (x2, y2) = nodes[j];

                // Calculate direction vector (dx, dy) and normalize
                let dx = x2 - x1;
                let dy = y2 - y1;
                let gcd = gcd(dx, dy);
                let dx = dx / gcd;
                let dy = dy / gcd;

                // Add all collinear points between the two antennas
                let mut x = x1;
                let mut y = y1;

                while x >= 0 && x < width as i32 && y >= 0 && y < height as i32 {
                    antinodes.insert((x, y));
                    x -= dx;
                    y -= dy;
                }

                x = x1 + dx;
                y = y1 + dy;

                while x >= 0 && x < width as i32 && y >= 0 && y < height as i32 {
                    antinodes.insert((x, y));
                    x += dx;
                    y += dy;
                }
            }
        }
    }

    println!("Total unique antinode positions: {}", antinodes.len());
}

// Helper function to calculate GCD
fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a.abs()
}

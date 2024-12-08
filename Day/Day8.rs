use std::collections::HashSet;
use std::fs;

fn main() {
    
    let filename = "Day7.txt";
    let map_content = fs::read_to_string(filename).expect("Failed to read input file");
    let map: Vec<&str> = map_content.lines().collect();
    let rows = map.len();
    let cols = map[0].len();
    let mut antennas = vec![];
    for (y, row) in map.iter().enumerate() {
        for (x, c) in row.chars().enumerate() {
            if c.is_alphanumeric() {
                antennas.push((x as isize, y as isize, c));
            }
        }
    }
    let mut antinodes: HashSet<(isize, isize)> = HashSet::new();
    for i in 0..antennas.len() {
        for j in (i + 1)..antennas.len() {
            let (x1, y1, freq1) = antennas[i];
            let (x2, y2, freq2) = antennas[j];

            // Check if the frequencies match
            if freq1 == freq2 {
                // Calculate distance between the antennas
                let dx = x2 - x1;
                let dy = y2 - y1;

                // Antinodes occur when one antenna is twice as far as the other
                let antinode1 = (x1 - dx, y1 - dy);
                let antinode2 = (x2 + dx, y2 + dy);

                // Add valid antinodes within bounds
                if antinode1.0 >= 0 && antinode1.0 < cols as isize && antinode1.1 >= 0 && antinode1.1 < rows as isize {
                    antinodes.insert(antinode1);
                }
                if antinode2.0 >= 0 && antinode2.0 < cols as isize && antinode2.1 >= 0 && antinode2.1 < rows as isize {
                    antinodes.insert(antinode2);
                }
            }
        }
    }

    // Count unique antinodes
    println!("Unique antinodes within bounds: {}", antinodes.len());
}

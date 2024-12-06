use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Stdout, Write};
use std::collections::HashSet;
use std::path::Path;

const DIR8: [(i32, i32); 8] = [
    (1, 0),
    (0, 1),
    (-1, 0),
    (0, -1),
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
];

fn solve(input: &str, writer: &mut BufWriter<Stdout>) {
    let mut map: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut start: (i32, i32) = (-1, -1);
    let mut dir: (i32, i32) = (-1, -1);

    // Find the starting point and initial direction
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            match map[i][j] {
                '>' => {
                    start = (i as i32, j as i32);
                    dir = (0, 1);
                }
                '<' => {
                    start = (i as i32, j as i32);
                    dir = (0, -1);
                }
                'v' => {
                    start = (i as i32, j as i32);
                    dir = (1, 0);
                }
                '^' => {
                    start = (i as i32, j as i32);
                    dir = (-1, 0);
                }
                _ => (),
            }
        }
    }

    // Mark initial visited positions
    let mut initial_visited: Vec<Vec<bool>> = vec![vec![false; map[0].len()]; map.len()];
    let mut initial_dir = dir;
    travel_map(
        start.0,
        start.1,
        &mut initial_dir,
        &map,
        &mut initial_visited,
    );

    let mut cycles: i64 = 0;
    for i in 0..initial_visited.len() {
        for j in 0..initial_visited[0].len() {
            if (i as i32, j as i32) == start || map[i][j] != '.' {
                continue;
            }

            let mut is_surrounding = false;
            for &(dx, dy) in DIR8.iter() {
                let (x, y) = ((i as i32) + dx, (j as i32) + dy);
                if x >= 0
                    && x < initial_visited.len() as i32
                    && y >= 0
                    && y < initial_visited[0].len() as i32
                    && initial_visited[x as usize][y as usize]
                {
                    is_surrounding = true;
                    break;
                }
            }
            if !is_surrounding {
                continue;
            }

            map[i][j] = '#';

            let mut cur_visited: HashSet<(usize, usize, i32, i32)> = HashSet::new();
            let mut cur_dir = dir;
            let mut has_cycle = false;

            check_cycle_in_map(
                start.0,
                start.1,
                &mut cur_dir,
                &map,
                &mut cur_visited,
                &mut has_cycle,
            );
            if has_cycle {
                cycles += 1;
            }

            map[i][j] = '.';
        }
    }

    writeln!(writer, "{}", cycles).ok();
}

fn check_cycle_in_map(
    x: i32,
    y: i32,
    dir: &mut (i32, i32),
    map: &Vec<Vec<char>>,
    visited: &mut HashSet<(usize, usize, i32, i32)>,
    has_cycle: &mut bool,
) {
    if !visited.contains(&(x as usize, y as usize, dir.0, dir.1)) {
        visited.insert((x as usize, y as usize, dir.0, dir.1));
    } else {
        *has_cycle = true;
        return;
    }

    let (mut nx, mut ny) = (x + dir.0, y + dir.1);
    if nx >= 0 && nx < map.len() as i32 && ny >= 0 && ny < map[0].len() as i32 {
        if map[nx as usize][ny as usize] == '#' {
            if dir.0 != 0 {
                dir.1 = -dir.0;
                dir.0 = 0;
            } else {
                dir.0 = dir.1;
                dir.1 = 0;
            }
            (nx, ny) = (x, y);
        }
        check_cycle_in_map(nx, ny, dir, map, visited, has_cycle);
    }
}

fn travel_map(
    x: i32,
    y: i32,
    dir: &mut (i32, i32),
    map: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
) {
    visited[x as usize][y as usize] = true;

    let (mut nx, mut ny) = (x + dir.0, y + dir.1);
    if nx >= 0 && nx < map.len() as i32 && ny >= 0 && ny < map[0].len() as i32 {
        if map[nx as usize][ny as usize] == '#' {
            if dir.0 != 0 {
                dir.1 = -dir.0;
                dir.0 = 0;
            } else {
                dir.0 = dir.1;
                dir.1 = 0;
            }
            (nx, ny) = (x, y);
        }
        travel_map(nx, ny, dir, map, visited);
    }
}

fn main() {
    let path = Path::new("data6.txt");
    let file = File::open(&path).expect("Failed to open input file");
    let mut reader = BufReader::new(file);
    let mut input = String::new();
    reader.read_to_string(&mut input).expect("Failed to read file");

    let writer = &mut BufWriter::new(io::stdout());
    solve(&input, writer);
}

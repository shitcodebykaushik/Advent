use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};

const DIRECTIONS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)]; // Right, Down, Left, Up

type IVec2 = (isize, isize);

fn parse_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn find_node(grid: &[Vec<char>], target: char) -> IVec2 {
    grid.iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter()
                .enumerate()
                .find(|(_, &c)| c == target)
                .map(|(x, _)| (x as isize, y as isize))
        })
        .unwrap()
}

/// Traverse grid to find the minimum cost from start to end using Dijkstra's Algorithm.
fn traverse(grid: &[Vec<char>], start: IVec2, end: IVec2) -> usize {
    let mut queue: BinaryHeap<Reverse<(usize, IVec2, IVec2)>> = BinaryHeap::new();
    let mut seen: HashSet<(IVec2, IVec2)> = HashSet::new();
    queue.push(Reverse((0, start, (1, 0)))); // Start facing East

    while let Some(Reverse((cost, pos, dir))) = queue.pop() {
        if seen.contains(&(pos, dir)) {
            continue;
        }
        if pos == end {
            return cost;
        }
        seen.insert((pos, dir));

        // Move forward
        let forwards = (pos.0 + dir.0, pos.1 + dir.1);
        if grid[forwards.1 as usize][forwards.0 as usize] != '#' {
            queue.push(Reverse((cost + 1, forwards, dir)));
        }

        // Turn left
        let left = (dir.1, -dir.0);
        queue.push(Reverse((cost + 1000, pos, left)));

        // Turn right
        let right = (-dir.1, dir.0);
        queue.push(Reverse((cost + 1000, pos, right)));
    }

    unreachable!()
}

/// Recursive function to walk back and find all tiles part of the best paths.
fn walk(
    cur: Option<(IVec2, IVec2)>,
    routes: &mut HashSet<(IVec2, IVec2)>,
    tiles: &mut HashSet<IVec2>,
    links: &HashMap<(IVec2, IVec2), HashSet<Option<(IVec2, IVec2)>>>,
) {
    if let Some(cur) = cur {
        if !routes.contains(&cur) {
            routes.insert(cur);
            tiles.insert(cur.0);

            if let Some(link) = links.get(&cur) {
                for &pos in link {
                    walk(pos, routes, tiles, links);
                }
            }
        }
    }
}

/// Backtrack to determine all tiles that are part of any of the best paths.
fn traverse_all(grid: &[Vec<char>], start: IVec2, end: IVec2, target_cost: usize) -> usize {
    let mut best_costs: HashMap<(IVec2, IVec2), usize> = HashMap::new();
    let mut links: HashMap<(IVec2, IVec2), HashSet<Option<(IVec2, IVec2)>>> = HashMap::new();
    let mut queue: BinaryHeap<Reverse<(usize, IVec2, IVec2, Option<(IVec2, IVec2)>)>> =
        BinaryHeap::new();
    queue.push(Reverse((0, start, (1, 0), None))); // Start facing East

    while let Some(Reverse((cost, pos, dir, prev))) = queue.pop() {
        if cost > target_cost {
            break;
        }

        if best_costs.contains_key(&(pos, dir)) {
            if cost == best_costs[&(pos, dir)] {
                links.entry((pos, dir)).or_default().insert(prev);
            }
            continue;
        }

        best_costs.insert((pos, dir), cost);
        links.entry((pos, dir)).or_default().insert(prev);

        let prev = Some((pos, dir));

        // Move forward
        let forwards = (pos.0 + dir.0, pos.1 + dir.1);
        if grid[forwards.1 as usize][forwards.0 as usize] != '#' {
            queue.push(Reverse((cost + 1, forwards, dir, prev)));
        }

        // Turn left
        let left = (dir.1, -dir.0);
        queue.push(Reverse((cost + 1000, pos, left, prev)));

        // Turn right
        let right = (-dir.1, dir.0);
        queue.push(Reverse((cost + 1000, pos, right, prev)));
    }

    let mut routes: HashSet<(IVec2, IVec2)> = HashSet::new();
    let mut tiles: HashSet<IVec2> = HashSet::new();

    for dir in &DIRECTIONS {
        walk(Some((end, *dir)), &mut routes, &mut tiles, &links);
    }

    tiles.len()
}

/// Solve both parts of the problem.
fn solve(input: &str, part_two: bool) -> usize {
    let grid = parse_grid(input);
    let start = find_node(&grid, 'S');
    let end = find_node(&grid, 'E');

    let cost = traverse(&grid, start, end);

    if part_two {
        traverse_all(&grid, start, end, cost)
    } else {
        cost
    }
}

fn main() {
    const TEST_INPUT: &str = "###############\n#.......#....E#\n#.#.###.#.###.#\n#.....#.#...#.#\n#.###.#####.#.#\n#.#.#.......#.#\n#.#.#####.###.#\n#...........#.#\n###.#.#####.#.#\n#...#.....#.#.#\n#.#.#.###.#.#.#\n#.....#...#.#.#\n#.###.#.#.#.#.#\n#S..#.....#...#\n###############";
    const INPUT: &str = include_str!("../Day7.txt");

    // Part 1 Test and Solution
    assert_eq!(solve(TEST_INPUT, false), 7036);
    println!("Day 16 Part One: {}", solve(INPUT, false));

    // Part 2 Test and Solution
    assert_eq!(solve(TEST_INPUT, true), 45);
    println!("Day 16 Part Two: {}", solve(INPUT, true));
}

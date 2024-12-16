use std::fs::read_to_string;
use std::io;

#[derive(Debug, Copy, Clone)]
enum Thing {
    Wall,
    Robot,
    End,
    Empty,
}

#[derive(Debug, Copy, Clone)]
struct Item {
    x: i32,
    y: i32,
    thing: Thing,
}

type Map = Vec<Vec<Item>>;

fn parse_map(content: &str) -> (Map, Item, Item) {
    let lines: Vec<&str> = content.lines().collect();
    let height = lines.len();
    let width = lines[0].len();
    let mut robot = Item {
        x: 0,
        y: 0,
        thing: Thing::Robot,
    };
    let mut end = Item {
        x: 0,
        y: 0,
        thing: Thing::Empty,
    };
    let mut map: Vec<Vec<Item>> = Vec::with_capacity(height);

    for (y, line) in lines.iter().enumerate() {
        map.push(Vec::with_capacity(width));
        for (x, ch) in line.chars().enumerate() {
            if ch == 'S' {
                robot = Item {
                    x: x as i32,
                    y: y as i32,
                    thing: Thing::Robot,
                };
            }
            map[y].push(Item {
                x: x as i32,
                y: y as i32,
                thing: match ch {
                    '#' => Thing::Wall,
                    'S' => Thing::Robot,
                    'E' => Thing::End,
                    _ => Thing::Empty,
                },
            });
            if ch == 'E' {
                end = map[y][map[y].len() - 1];
            }
        }
    }

    (map, robot, end)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct State {
    cost: i32,
    position: (i32, i32),
    direction: Direction,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn turn_left(dir: Direction) -> Direction {
    match dir {
        Direction::North => Direction::West,
        Direction::West => Direction::South,
        Direction::South => Direction::East,
        Direction::East => Direction::North,
    }
}

fn turn_right(dir: Direction) -> Direction {
    match dir {
        Direction::North => Direction::East,
        Direction::East => Direction::South,
        Direction::South => Direction::West,
        Direction::West => Direction::North,
    }
}

fn get_neighbors(
    pos: (i32, i32),
    current_dir: Direction,
    is_first_move: bool,
    map: &Map,
) -> Vec<((i32, i32), Direction)> {
    let mut neighbors = Vec::new();
    let valid_turns = match is_first_move {
        true => vec![current_dir, turn_left(current_dir), turn_right(current_dir)],
        false => vec![current_dir, turn_left(current_dir), turn_right(current_dir)],
    };

    for new_dir in valid_turns {
        let (dx, dy) = match new_dir {
            Direction::North => (0, -1),
            Direction::South => (0, 1),
            Direction::East => (1, 0),
            Direction::West => (-1, 0),
        };
        let new_x = pos.0 + dx;
        let new_y = pos.1 + dy;

        if new_y >= 0
            && (new_y as usize) < map.len()
            && new_x >= 0
            && (new_x as usize) < map[0].len()
        {
            if !matches!(map[new_y as usize][new_x as usize].thing, Thing::Wall) {
                neighbors.push(((new_x, new_y), new_dir));
            }
        }
    }
    neighbors
}

fn find_path(map: &Map, start: Item, end: Item) -> Option<(i32, Vec<(i32, i32)>)> {
    use std::collections::{BinaryHeap, HashMap};

    let mut heap = BinaryHeap::new();
    let mut costs = HashMap::new();

    let initial_state = State {
        cost: 0,
        position: (start.x, start.y),
        direction: Direction::East,
    };
    heap.push(initial_state);
    costs.insert(((start.x, start.y), Direction::East), 0);

    let mut came_from: HashMap<((i32, i32), Direction), ((i32, i32), Direction)> = HashMap::new();

    while let Some(State {
        cost,
        position,
        direction,
    }) = heap.pop()
    {
        if position == (end.x, end.y) {
            let mut path = Vec::new();
            let mut current_state = (position, direction);
            path.push(current_state.0);

            while let Some(&prev_state) = came_from.get(&current_state) {
                path.push(prev_state.0);
                current_state = prev_state;

                if current_state.0 == (start.x, start.y) {
                    break;
                }
            }

            path.reverse();
            return Some((cost, path));
        }

        let is_first_move = position == (start.x, start.y);
        for (next_pos, next_dir) in get_neighbors(position, direction, is_first_move, map) {
            let move_cost = if direction == next_dir { 1 } else { 1001 };
            let next_cost = cost + move_cost;

            let current_best = costs.get(&(next_pos, next_dir));
            let is_better = current_best.map_or(true, |&c| next_cost < c);

            if is_better {
                costs.insert((next_pos, next_dir), next_cost);
                came_from.insert((next_pos, next_dir), (position, direction));
                heap.push(State {
                    cost: next_cost,
                    position: next_pos,
                    direction: next_dir,
                });
            }
        }
    }

    None
}

fn main() -> io::Result<()> {
    let content = read_to_string("Day7.txt")?;
    let (map, robot, end) = parse_map(&content);

    println!(
        "Starting search from {:?} to {:?}",
        (robot.x, robot.y),
        (end.x, end.y)
    );
    println!("Map dimensions: {}x{}", map.len(), map[0].len());

    match find_path(&map, robot, end) {
        Some((cost, path)) => {
            println!("Shortest path cost: {}", cost);
            println!("Path: {:?}", path);
        }
        None => {
            println!("No path found!");
        }
    }

    Ok(())
}

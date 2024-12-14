
fn main() {
    let input = std::fs::read_to_string("Day7.txt").unwrap();

    println!("Part 1: {}", solve(&input));
    part2(&input);
}

type PosType = i16;
type VelType = i16;

const WIDTH: PosType = 101;
const HEIGHT: PosType = 103;

#[derive(Debug, Copy, Clone)]
struct Robot {
    x: PosType,
    y: PosType,
    vx: VelType,
    vy: VelType,
}

impl Robot {
    #[inline(always)]
    fn move_once(&mut self) {
        self.x += self.vx as PosType;
        self.y += self.vy as PosType;
        if self.x < 0 {
            self.x += WIDTH;
        } else if self.x >= WIDTH {
            self.x -= WIDTH;
        }
        if self.y < 0 {
            self.y += HEIGHT;
        } else if self.y >= HEIGHT {
            self.y -= HEIGHT;
        }
    }
}

fn solve(input: &String) -> usize {
    let start = std::time::Instant::now();

    let mut robots = parse_input(&input);
    for _ in 0..100 {
        for robot in &mut robots {
            robot.move_once();
        }
    }

    let mut quads = [0usize; 4];
    for robot in &robots {
        if robot.x == WIDTH / 2 {
            continue;
        }
        if robot.y == HEIGHT / 2 {
            continue;
        }

        let x = robot.x / (WIDTH / 2 + 1);
        let y = robot.y / (HEIGHT / 2 + 1);
        quads[((x as usize) << 1) | y as usize] += 1;
    }
    let safety_factor = quads[0] * quads[1] * quads[2] * quads[3];

    let time = start.elapsed();
    println!("Time: {:?}", &time);

    safety_factor
}

// THIS IS NOT A SOLUTION
// this is merely a tool to help find it
// this finds the first time at which no two robots occupy the same space
fn part2(input: &String) {
    let mut robots = parse_input(&input);

    let mut s = 0;
    'outer: loop {
        s += 1;
        for robot in &mut robots {
            robot.move_once();
        }

        for i in 0..(robots.len() - 1) {
            for j in (i + 1)..robots.len() {
                if robots[i].x == robots[j].x && robots[i].y == robots[j].y {
                    continue 'outer;
                }
            }
        }
        println!("s: {s}");
        break;
    }
}

fn parse_input(input: &String) -> Vec<Robot> {
    input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(' ').expect("invalid input");
            let (x, y) = left["p=".len()..].split_once(',').expect("invalid input");
            let (vx, vy) = right["v=".len()..].split_once(',').expect("invalid input");
            let x = x.parse::<PosType>().expect("invalid input");
            let y = y.parse::<PosType>().expect("invalid input");
            let vx = vx.parse::<VelType>().expect("invalid input");
            let vy = vy.parse::<VelType>().expect("invalid input");
            Robot { x, y, vx, vy }
        })
        .collect::<Vec<_>>()
}

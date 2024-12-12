use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl std::ops::Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    fn get(&self, grid : &Vec<Vec<char>>) -> char {
        grid[self.x as usize][self.y as usize]
    }

    fn rotate_right(&self, num : i32) -> Point {
        match num {
            0 => Point::new(self.x, self.y),
            1 => Point::new(self.y, -self.x),
            2 => Point::new(-self.x, -self.y),
            3 => Point::new(-self.y, self.x),
            _ => panic!("Invalid rotation"),
        }
    }

    fn neighbours(&self, grid : &Vec<Vec<char>>) -> Vec<Point> {
        let mut neighbours = Vec::new();
        if self.x > 0 {
            neighbours.push(Point::new(self.x - 1, self.y));
        }
        if self.x < (grid.len() - 1) as i32 {
            neighbours.push(Point::new(self.x + 1, self.y));
        }
        if self.y > 0 {
            neighbours.push(Point::new(self.x, self.y - 1));
        }
        if self.y < (grid[0].len() - 1) as i32 {
            neighbours.push(Point::new(self.x, self.y + 1));
        }
        neighbours
    }

    fn in_bounds(&self, grid : &Vec<Vec<char>>) -> bool {
        self.x >= 0 && self.x < grid.len() as i32 && self.y >= 0 && self.y < grid[0].len() as i32
    }

    fn perimeter_at_point(&self, grid : &Vec<Vec<char>>) -> usize {
        4 - self.neighbours(grid).iter().filter(|&p| p.get(grid) == self.get(grid)).count()
    }

    fn sides(&self, grid : &Vec<Vec<char>>, visited:  &HashSet<Point>) -> usize {
        let edges = vec![Point::new(0, 1), Point::new(1, 0), Point::new(0, -1), Point::new(-1, 0)]
            .iter()
            .filter(|&dir| self.side_in_dir(grid, visited, *dir))
            .count();

        edges
    }

    fn side_in_dir(self, grid : &Vec<Vec<char>>, visited : &HashSet<Point>, dir : Point) -> bool {
        // Tile has no fence in this direction
        if (self + dir).in_bounds(grid) && (self + dir).get(grid) == self.get(grid) {
            return false;
        }


        let left = dir.rotate_right(3);
        let right = dir.rotate_right(1);

        // Has a left neighbour that has been counted
        if (self + left).in_bounds(grid) && (self + left).get(grid) == self.get(grid) && visited.contains(&(self + left)) {
            // That neighbour already has a side counted
            if !(self + left + dir).in_bounds(grid) || (self + left + dir).get(grid) != self.get(grid) {
                return false;
            }
        }

        // Has a right neighbour that has been counted
        if (self + right).in_bounds(grid) && (self + right).get(grid) == self.get(grid) && visited.contains(&(self + right)) {
            // That neighbour already has a side counted
            if !(self + right + dir).in_bounds(grid) || (self + right + dir).get(grid) != self.get(grid) {
                return false;
            }
        }
        true
    }
}

#[derive(Debug)]
struct Region {
    letter : char,
    perimeter: usize,
    sides: usize,
    area: usize,
}

fn main() {
    let file = include_str!("../Day7.txt");

    let mut visited : HashSet<Point> = HashSet::new();
    let grid : Vec<Vec<char>> = file.lines().map(|line| line.chars().collect()).collect();
    let mut regions : Vec<Region> = Vec::new();

    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            let point = Point::new(x as i32, y as i32);
            if visited.contains(&point) {
                continue;
            }
            let mut queue = vec![point];
            let mut region = Region {
                letter: point.get(&grid),
                perimeter: 0,
                sides: 0,
                area: 0,
            };
            while let Some(p) = queue.pop() {
                if visited.contains(&p) {
                    continue;
                }
                visited.insert(p);
                region.area += 1;
                region.perimeter += p.perimeter_at_point(&grid);
                region.sides += p.sides(&grid, &visited);
                for neighbour in p.neighbours(&grid) {
                    if !visited.contains(&neighbour) && neighbour.get(&grid) == p.get(&grid) {
                        queue.insert(0, neighbour);
                    }
                }
            }
            regions.push(region);
        }
    }

    let sum = regions.iter().map(|r| r.area * r.perimeter).sum::<usize>();

    for region in regions.iter_mut() {
        println!("{}: {} area {} sides {} perimeter", region.letter, region.area, region.sides, region.perimeter);
    
        if region.sides % 2 == 1 {
            println!("{} has an odd number of sides, we shall correct", region.letter);
            region.sides -= 1;
        }
    }
    println!("PART 1: {}", sum);

    println!("PART 2: {}", regions.iter().map(|r| r.area * r.sides).sum::<usize>());
}
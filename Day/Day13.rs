use std::fs::File;
use std::io::{self, Read};
use nom::{
    bytes::complete::tag, 
    character::complete::{digit1, newline}, 
    combinator::{opt, map}, 
    multi::many0, 
    sequence::{preceded, terminated, tuple}, 
    IResult,
};

#[derive(Debug, Clone)]
struct ClawMachine {
    ax: i128,
    ay: i128,
    bx: i128,
    by: i128,
    px: i128,
    py: i128,
}

impl ClawMachine {
    /// Solve for the minimum cost to align the claw to the prize
    fn solve(&self, press_limit: u64) -> Option<u64> {
        let determinant = self.ay * self.bx - self.ax * self.by;
        if determinant == 0 {
            return None; // No solution if determinant is zero
        }

        let a_frac = (
            -1 * (self.by * self.px - self.bx * self.py),
            determinant,
        );
        let b_frac = (
            self.ay * self.px - self.ax * self.py,
            determinant,
        );

        if a_frac.0 % a_frac.1 != 0 || b_frac.0 % b_frac.1 != 0 {
            return None; // Ensure solutions are integers
        }

        let a_tries = (a_frac.0 / a_frac.1) as u64;
        let b_tries = (b_frac.0 / b_frac.1) as u64;

        if a_tries > press_limit || b_tries > press_limit {
            return None; // Validate press limits
        }

        Some(3 * a_tries + b_tries) // Total cost
    }
}

/// Parse a single claw machine configuration
fn claw_machine_parser(input: &str) -> IResult<&str, ClawMachine> {
    map(
        preceded(
            opt(many0(newline)),
            tuple((
                preceded(tag("Button A: X+"), digit1),
                terminated(preceded(tag(", Y+"), digit1), newline),
                preceded(tag("Button B: X+"), digit1),
                terminated(preceded(tag(", Y+"), digit1), newline),
                preceded(tag("Prize: X="), digit1),
                terminated(preceded(tag(", Y="), digit1), opt(newline)),
            )),
        ),
        |(ax, ay, bx, by, px, py): (&str, &str, &str, &str, &str, &str)| {
            ClawMachine {
                ax: ax.parse::<i128>().unwrap(),
                ay: ay.parse::<i128>().unwrap(),
                bx: bx.parse::<i128>().unwrap(),
                by: by.parse::<i128>().unwrap(),
                px: px.parse::<i128>().unwrap(),
                py: py.parse::<i128>().unwrap(),
            }
        },
    )(input)
}

/// Parse the input file to extract all claw machines
fn parse_input(input: &str) -> Vec<ClawMachine> {
    let mut input = input;
    let mut machines = vec![];

    while !input.is_empty() {
        match claw_machine_parser(input) {
            Ok((rest, claw_machine)) => {
                input = rest;
                machines.push(claw_machine);
            }
            Err(_) => break,
        }
    }

    machines
}

/// Process Part 1: Solve for the minimum total cost for all machines
pub fn process_part1(input: &str) -> String {
    parse_input(input)
        .iter()
        .map(|claw_machine| claw_machine.solve(100u64).unwrap_or(0))
        .sum::<u64>()
        .to_string()
}

/// Main function
fn main() -> io::Result<()> {
    // Load input from file
    let mut file = File::open("Day7.txt")?; // Replace "input.txt" with your actual file path
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    // Process Part 1
    let result = process_part1(&input);
    println!("Total cost for Part 1: {}", result);

    Ok(())
}

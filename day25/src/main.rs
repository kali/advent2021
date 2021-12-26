use std::{convert::Infallible, fmt::Debug, str::FromStr};

fn main() {
    let p1 = run(&std::fs::read_to_string("input").unwrap());
    dbg!(p1);
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Cell {
    Empty,
    South,
    East,
}
use Cell::*;

#[derive(Clone, PartialEq)]
struct State(Vec<Vec<Cell>>);

impl FromStr for State {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = s
            .trim()
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        'v' => Cell::South,
                        '>' => Cell::East,
                        '.' => Cell::Empty,
                        _ => panic!(),
                    })
                    .collect()
            })
            .collect();
        Ok(State(grid))
    }
}

impl Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        for row in &self.0 {
            for col in row {
                let c = match col {
                    Empty => '.',
                    South => 'v',
                    East => '>',
                };
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl State {
    fn step(&self) -> State {
        let mut east = self.clone();
        for row in 0..self.0.len() {
            for col in 0..self.0[0].len() {
                let dst = (col + 1) % self.0[0].len();
                if self.0[row][col] == East && self.0[row][dst] == Empty {
                    east.0[row][col] = Empty;
                    east.0[row][dst] = East;
                }
            }
        }
        let mut south = east.clone();
        for row in 0..self.0.len() {
            for col in 0..self.0[0].len() {
                let dst = (row + 1) % self.0.len();
                if east.0[row][col] == South && east.0[dst][col] == Empty {
                    south.0[row][col] = Empty;
                    south.0[dst][col] = South;
                }
            }
        }
        south
    }
}

fn run(input: &str) -> usize {
    let mut previous: State = input.parse().unwrap();
    let mut step = 0;
    loop {
        let current = previous.step();
        step += 1;
        if previous == current {
            return step;
        }
        previous = current;
    }
}

#[test]
fn t() {
    assert_eq!(
        run("
v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>
"),
        59
    );
}

#[test]
fn t_step() {
    assert_eq!(
        State::from_str(
            "
v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>
"
        )
        .unwrap()
        .step(),
        State::from_str(
            "
....>.>v.>
v.v>.>v.v.
>v>>..>v..
>>v>v>.>.v
.>v.v...v.
v>>.>vvv..
..v...>>..
vv...>>vv.
>.v.v..v.v
"
        )
        .unwrap()
    );
}

#[test]
fn t_step_simple() {
    assert_eq!(
        State::from_str(
            "
..........
.>v....v..
.......>..
..........
"
        )
        .unwrap()
        .step(),
        State::from_str(
            "
..........
.>........
..v....v>.
..........
"
        )
        .unwrap()
    );
}

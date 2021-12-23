use std::{convert::Infallible, str::FromStr};

use pathfinding::directed::astar;

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    dbg!(run(input.trim()));
    dbg!(run(&unfold(input.trim())));
}

#[derive(Clone, PartialEq, Hash, Eq, Debug)]
struct State(Vec<(i8, i8)>);

impl FromStr for State {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pods = s.replace(&[' ', '\n', '.', '#'] as &[char], "");
        let mut state = vec![(0, 0); pods.len()];
        for (ix, pod) in pods.bytes().enumerate() {
            let kind = (pod - b'A') as usize;
            let (x, y) = (home_x(ix as i8 % 4), 2 + (ix / 4) as i8);
            let state_ix = kind * pods.len() / 4;
            for t in state_ix.. {
                if state[t] == (0, 0) {
                    state[t] = (x, y);
                    break;
                }
            }
        }
        Ok(State(state))
    }
}

fn home_x(kind: i8) -> i8 {
    kind * 2 + 3
}

fn cost(kind: i8) -> usize {
    [1, 10, 100, 1000][kind as usize]
}

impl State {
    fn at(&self, x: i8, y: i8) -> Option<i8> {
        self.0
            .iter()
            .position(|&(x1, y1)| x == x1 && y == y1)
            .map(|ix| self.kind(ix as _))
    }

    fn kind(&self, ix: i8) -> i8 {
        ix / (self.0.len() as i8 / 4)
    }

    fn can_move_in(&self, kind: i8) -> Option<i8> {
        let home_x = home_x(kind);
        let max_y = (2 + self.0.len() / 4) as i8;
        for y in 2..max_y {
            if let Some(k) = self.at(home_x, y) {
                if k != kind {
                    return None;
                }
            }
        }
        for y in (2..max_y).rev() {
            if self.at(home_x, y).is_none() {
                return Some(y);
            }
        }
        unreachable!();
    }

    fn hallway_free(&self, from: i8, to: i8) -> bool {
        let range = if from < to {
            from + 1..to + 1
        } else {
            to..from
        };
        !self.0.iter().any(|(x, y)| *y == 1 && range.contains(&x))
    }

    fn next(&self) -> Vec<(State, usize)> {
        let mut moves = vec![];
        for (ix, (x, y)) in self.0.iter().copied().enumerate() {
            let kind = self.kind(ix as i8);
            if y == 1 {
                if let Some(home_y) = self.can_move_in(kind) {
                    if self.hallway_free(x, home_x(kind)) {
                        let mut state = self.clone();
                        state.0[ix] = (home_x(kind), home_y);
                        let dist = home_y - 1 + (home_x(kind) - x).abs();
                        moves.push((state, dist as usize * cost(kind)));
                    }
                }
            } else if y == 2 || self.at(x, 2).is_none() {
                for to_x in [1, 2, 4, 6, 8, 10, 11] {
                    if self.hallway_free(x, to_x) && (1..y).all(|y| self.at(x, y).is_none()) {
                        let mut state = self.clone();
                        state.0[ix] = (to_x, 1);
                        let dist = y - 1 + (to_x - x).abs();
                        moves.push((state, dist as usize * cost(kind)));
                    }
                }
            }
        }
        moves
    }

    fn success(&self) -> bool {
        self.0.iter().all(|&(_, y)| y > 1)
            && self
                .0
                .iter()
                .enumerate()
                .all(|(ix, (x, _))| *x == home_x(self.kind(ix as i8)))
    }

    fn heuristic(&self) -> usize {
        self.0
            .iter()
            .enumerate()
            .map(|(ix, (x, _))| {
                let kind = self.kind(ix as i8);
                (home_x(kind) - x).abs() as usize * cost(kind)
            })
            .sum::<usize>()
    }

    fn dump(&self) {
        let y_max = self.0.len() / 4 + 2;
        for y in 0..y_max as i8 {
            for x in 0..12 {
                if let Some(c) = self.at(x, y) {
                    print!("{}", (c as u8 + b'A') as char);
                } else {
                    print!(".");
                }
            }
            println!("");
        }
        println!("{}\n", self.heuristic());
    }
}

fn unfold(input: &str) -> String {
    let mut lines: Vec<_> = input.lines().map(|s| s.to_owned()).collect();
    lines.insert(3, "#D#C#B#A# #D#B#A#C#".to_string());
    lines.into_iter().collect()
}

fn run(input: &str) -> usize {
    let state = input.parse::<State>().unwrap();
    let res = astar::astar(&state, State::next, State::heuristic, State::success).unwrap();
    for s in res.0 {
        s.dump();
        println!("");
    }
    res.1
}

#[test]
fn t() {
    assert_eq!(
        run("#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########
"),
        12521
    );
}

#[test]
fn t2() {
    let augmented = unfold(
        "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########
",
    );
    augmented.parse::<State>().unwrap().dump();
    assert_eq!(run(&augmented), 44169);
}

#[test]
fn success() {
    assert!("#############
#...........#
###A#B#C#D###
  #A#B#C#D#
  #########
"
    .parse::<State>()
    .unwrap()
    .success());
}

use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    dbg!(part1(input.trim()));
    dbg!(part2(input.trim()));
}

fn parse(input: &str) -> (usize, usize) {
    scan_fmt::scan_fmt!(
        input,
        "Player 1 starting position: {}\nPlayer 2 starting position: {}",
        usize,
        usize
    )
    .unwrap()
}

fn part1(input: &str) -> usize {
    let (mut pos1, mut pos2) = parse(input);
    let mut dice = 0;
    let mut roll = || {
        let res = dice % 100 + 1;
        dice += 1;
        res
    };
    let (mut score1, mut score2) = (0, 0);
    loop {
        pos1 = (pos1 + roll() + roll() + roll() - 1) % 10 + 1;
        score1 += pos1;
        if score1 >= 1000 {
            break;
        }
        pos2 = (pos2 + roll() + roll() + roll() - 1) % 10 + 1;
        score2 += pos2;
        if score2 >= 1000 {
            break;
        }
    }
    score1.min(score2) * dice
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct State {
    turn: usize,
    pos_1: usize,
    pos_2: usize,
    score_1: usize,
    score_2: usize,
}

fn part2(input: &str) -> usize {
    let (pos_1, pos_2) = parse(input);
    let start = State {
        turn: 0,
        pos_1,
        pos_2,
        score_1: 0,
        score_2: 0,
    };
    let mut paths_to_state = HashMap::<_, usize>::new();
    let mut queue = VecDeque::new();
    let mut queued = HashSet::new();
    let mut winner_1 = 0;
    let mut winner_2 = 0;
    queue.push_back(start);
    paths_to_state.insert(start, 1);
    while let Some(current) = queue.pop_front() {
        let from = paths_to_state[&current];
        for roll in 1..=3 {
            let mut next = current.clone();
            if current.turn % 6 < 3 {
                next.pos_1 = (next.pos_1 + roll - 1) % 10 + 1;
                if current.turn % 6 == 2 {
                    next.score_1 += next.pos_1;
                }
            } else {
                next.pos_2 = (next.pos_2 + roll - 1) % 10 + 1;
                if current.turn % 6 == 5 {
                    next.score_2 += next.pos_2;
                }
            }
            next.turn += 1;
            if next.score_1 >= 21 {
                winner_1 += from;
            } else if next.score_2 >= 21 {
                winner_2 += from;
            } else {
                *paths_to_state.entry(next).or_default() += from;
                if queued.insert(next) {
                    queue.push_back(next);
                }
            }
        }
    }
    winner_1.max(winner_2)
}

#[test]
fn t() {
    let input = "Player 1 starting position: 4
Player 2 starting position: 8";
    assert_eq!(part1(input), 739785);
    assert_eq!(part2(input), 444356092776315);
}

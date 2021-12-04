fn play(board: &[Vec<u8>], draw: &[u8]) -> (usize, u32) {
    for turn in 0.. {
        if board
            .iter()
            .any(|line| line.iter().all(|n| draw[..turn].contains(n)))
            || (0..5).any(|col| board.iter().all(|line| draw[..turn].contains(&line[col])))
        {
            let remains:u32 = board
                .iter()
                .map(|line| {
                    line.iter()
                        .filter(|n| !draw[..turn].contains(&n))
                        .map(|n| *n as u32)
                        .sum::<u32>()
                })
                .sum();
            let score = remains * draw[turn-1] as u32;
            return (turn, score);
        }
    }
    unreachable!();
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    dbg!(run(&input));
}

fn run(input:&str) -> (u32, u32) {
    let mut lines = input.lines();
    let draw: Vec<u8> = lines
        .next()
        .unwrap()
        .split(",")
        .map(|n| n.parse::<u8>().unwrap())
        .collect();
    let mut boards = vec![];
    let mut board = vec![];
    lines.for_each(|l| {
        if !l.trim().is_empty() {
            let nums: Vec<u8> = l.split_whitespace().map(|n| n.parse::<u8>().unwrap()).collect();
            board.push(nums);
            if board.len() == 5 {
                boards.push(board.clone());
                board.clear();
            }
        }
    });
    let winner = boards.iter().map(|b| play(&b, &draw)).min().unwrap();
    let looser = boards.iter().map(|b| play(&b, &draw)).max().unwrap();
    (winner.1, looser.1)
}

#[test]
fn t() {
    assert_eq!(run("7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
"), (4512,  1924));
}

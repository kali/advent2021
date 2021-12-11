fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    dbg!(run(input.trim()));
}

fn run(input: &str) -> (usize, usize) {
    let mut state: Vec<Vec<u8>> = input
        .lines()
        .map(|line| line.bytes().map(|c| c - b'0').collect())
        .collect();
    let mut p1 = 0;
    for step in 0.. {
        let mut todo = vec![];
        for row in 0..10 {
            for col in 0..10 {
                state[row][col] += 1;
                if state[row][col] == 10 {
                    todo.push((row, col));
                }
            }
        }
        while let Some((r, c)) = todo.pop() {
            for (dx, dy) in [
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1),
            ] {
                let (row, col) = (r as isize + dx, c as isize + dy);
                if row < 0 || row > 9 || col < 0 || col > 9 {
                    continue;
                }
                state[row as usize][col as usize] += 1;
                if state[row as usize][col as usize] == 10 {
                    todo.push((row as usize, col as usize));
                }
            }
        }
        let mut flashed = 0;
        for row in 0..10 {
            for col in 0..10 {
                if state[row][col] > 9 {
                    state[row][col] = 0;
                    flashed += 1;
                }
            }
        }
        if step < 100 {
            p1 += flashed;
        }
        if flashed == 100 {
            return (p1, step + 1)
        }
    }
    unreachable!()
}

#[test]
fn t() {
    assert_eq!(
        run("5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"),
        (1656, 195)
    );
}

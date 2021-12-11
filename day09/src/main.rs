use pathfinding::directed::bfs::bfs_reach;

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    dbg!(run(input.trim()));
}

fn run(input: &str) -> (usize, usize) {
    let board: Vec<Vec<u8>> = input
        .lines()
        .map(|line| line.bytes().map(|b| b - b'0').collect())
        .collect();
    let mut p1 = 0;
    let mut basins = vec![];
    let board = &board;
    for row in 0..board.len() {
        for col in 0..board[0].len() {
            if (row == 0 || board[row - 1][col] > board[row][col])
                && (row == board.len() - 1 || board[row + 1][col] > board[row][col])
                && (col == 0 || board[row][col - 1] > board[row][col])
                && (col == board[0].len() - 1 || board[row][col + 1] > board[row][col])
            {
                p1 += (board[row][col] + 1) as usize;
                let basin = bfs_reach((row, col), |(row, col)| {
                    let row = *row;
                    let col = *col;
                    [(-1isize, 0isize), (1, 0), (0, -1), (0, 1)]
                        .into_iter()
                        .map(move |(dr, dc)| (row as isize + dr, col as isize + dc))
                        .filter(|(r, c)| {
                            *r >= 0
                                && *c >= 0
                                && *r < board.len() as isize
                                && *c < board[0].len() as isize
                        })
                        .map(|(r, c)| (r as usize, c as usize))
                        .filter(move |(r, c)| {
                            board[*r][*c] != 9 && board[*r][*c] >= board[row][col]
                        })
                })
                .count();
                basins.push(basin);
            }
        }
    }
    basins.sort();
    (
        p1,
        basins.pop().unwrap() * basins.pop().unwrap() * basins.pop().unwrap(),
    )
}

#[test]
fn t() {
    assert_eq!(
        run("2199943210
3987894921
9856789892
8767896789
9899965678"),
        (15, 1134)
    );
}

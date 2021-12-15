use pathfinding::directed::dijkstra::dijkstra;

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    dbg!(run(input.trim()));
}

fn score(map: &[Vec<u8>], x: usize, y: usize) -> usize {
    let h = map.len();
    let w = map[0].len();
    let tile_x = x / w;
    let tile_y = y / h;
    let it = map[y % h][x % w] as usize + tile_x + tile_y;
    let it = (it - 1) % 9 + 1;
    it
}

fn run(input: &str) -> (usize, usize) {
    let map: Vec<Vec<u8>> = input
        .lines()
        .map(|l| l.bytes().map(|b| b - b'0').collect())
        .collect();
    let h = map.len();
    let w = map[0].len();

    let (_, p1) = dijkstra(
        &(0usize, 0usize),
        |&(x, y)| {
            [(1, 0), (-1, 0), (0, 1), (0, -1)]
                .into_iter()
                .map(move |(dx, dy)| (x as isize + dx, y as isize + dy))
                .filter(|&(x, y)| x >= 0 && x < w as isize && y > 0 && y < h as isize)
                .map(|(x, y)| (x as usize, y as usize))
                .map(|(x, y)| ((x, y), score(&map, x, y)))
        },
        |&(x, y)| x == h - 1 && y == w - 1,
    )
    .unwrap();
    let (_, p2) = dijkstra(
        &(0usize, 0usize),
        |&(x, y)| {
            [(1, 0), (-1, 0), (0, 1), (0, -1)]
                .into_iter()
                .map(move |(dx, dy)| (x as isize + dx, y as isize + dy))
                .filter(|&(x, y)| x >= 0 && x < 5 * w as isize && y > 0 && y < 5 * h as isize)
                .map(|(x, y)| (x as usize, y as usize))
                .map(|(x, y)| ((x, y), score(&map, x, y)))
        },
        |&(x, y)| x == w * 5 - 1 && y == h * 5 - 1,
    )
    .unwrap();
    (p1, p2)
}

#[test]
fn t() {
    assert_eq!(
        run("1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
"),
        (40, 315)
    );
}

#[test]
fn t_score() {
    assert_eq!(score(&[vec![8]], 0, 0), 8);
    assert_eq!(score(&[vec![8]], 1, 0), 9);
    assert_eq!(score(&[vec![8]], 2, 0), 1);
    assert_eq!(score(&[vec![8]], 3, 0), 2);
    assert_eq!(score(&[vec![8]], 4, 0), 3);
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    dbg!(run(input.trim()));
}

fn run(input: &str) -> (usize, usize) {
    let mut pos: Vec<i64> = input.split(",").map(|s| s.parse().unwrap()).collect();
    pos.sort();
    let median = pos[pos.len() / 2];
    let p1 = pos.iter().map(|p| (p - median).abs()).sum::<i64>() as usize;
    let min = *pos.iter().min().unwrap();
    let max = *pos.iter().max().unwrap();
    let p2 = (min..max)
        .map(|p| {
            pos.iter()
                .map(|x| (x - p).abs() * ((x - p).abs() + 1) / 2)
                .sum::<i64>()
        })
        .min()
        .unwrap();
    (p1, p2 as _)
}

#[test]
fn t() {
    assert_eq!(run("16,1,2,0,4,2,7,1,2,14"), (37, 168));
}

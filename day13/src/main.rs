use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    dbg!(run(input.trim()));
}

fn fold(dots: &HashSet<(usize, usize)>, folds: &[(char, usize)]) -> HashSet<(usize, usize)> {
    let mut folded: HashSet<(usize, usize)> = dots.clone();
    for (dir, len) in folds {
        if *dir == 'x' {
            folded = folded
                .iter()
                .map(|(x, y)| (if x > len { 2 * len - x } else { *x }, *y))
                .collect()
        } else {
            folded = folded
                .iter()
                .map(|(x, y)| (*x, if y > len { 2 * len - y } else { *y }))
                .collect()
        }
    }
    folded
}

fn run(input: &str) -> usize {
    let (dots_section, folds_section) = input.split_once("\n\n").unwrap();
    let dots: HashSet<(usize, usize)> = dots_section
        .lines()
        .map(|l| l.split_once(",").unwrap())
        .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
        .collect();
    let folds: Vec<(char, usize)> = folds_section
        .lines()
        .map(|l| scan_fmt::scan_fmt!(l, "fold along {}={}", char, usize).unwrap())
        .collect();
    let p1 = fold(&dots, &folds[0..1]);
    let folded = fold(&dots, &folds);
    let lines = folded.iter().map(|pair| pair.1).max().unwrap();
    let cols = folded.iter().map(|pair| pair.0).max().unwrap();
    for y in 0..lines + 1 {
        for x in 0..cols + 1 {
            print!(
                "{}",
                if folded.contains(&(x, y)) {
                    "\u{2588}\u{2588}"
                } else {
                    "  "
                }
            );
        }
        println!();
    }
    p1.len()
}

#[test]
fn t() {
    assert_eq!(
        run("6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5"),
        17
    );
}

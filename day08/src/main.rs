use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    dbg!(run(input.trim()));
}

fn run(input: &str) -> (usize, usize) {
    let p1 = input
        .lines()
        .map(|line| {
            line.split_once("|")
                .unwrap()
                .1
                .split_whitespace()
                .map(|f| f.trim().len())
                .filter(|f| [2, 3, 4, 7].contains(f))
                .count()
        })
        .sum();
    let mut p2 = 0;
    for line in input.lines() {
        let mut resolved = vec![""; 10];
        let (left, number) = line.split_once("|").unwrap();
        let left: Vec<String> = left
            .split_whitespace()
            .map(|s| s.chars().sorted().collect())
            .collect();
        resolved[1] = left.iter().find(|d| d.len() == 2).unwrap();
        resolved[7] = left.iter().find(|d| d.len() == 3).unwrap();
        resolved[4] = left.iter().find(|d| d.len() == 4).unwrap();
        resolved[8] = left.iter().find(|d| d.len() == 7).unwrap();
        fn contains(small: &str, big: &str) -> bool {
            small.chars().all(|c| big.contains(c))
        }
        // 6 segments: 0,6,9. 9 contains 4, then 0 contains 7, last one is 6
        resolved[9] = left
            .iter()
            .find(|d| d.len() == 6 && contains(resolved[4], d))
            .unwrap();
        resolved[0] = left
            .iter()
            .find(|d| d.len() == 6 && d != &resolved[9] && contains(resolved[7], d))
            .unwrap();
        resolved[6] = left
            .iter()
            .find(|d| d != &resolved[9] && d != &resolved[0] && d.len() == 6)
            .unwrap();
        // 5 segments: 2,3,5. 3 contains 1, 6 contains 5
        resolved[3] = left
            .iter()
            .find(|d| d.len() == 5 && contains(resolved[1], d))
            .unwrap();
        resolved[5] = left
            .iter()
            .find(|d| d.len() == 5 && contains(d, resolved[6]))
            .unwrap();
        resolved[2] = left
            .iter()
            .find(|d| d.len() == 5 && d != &resolved[3] && d != &resolved[5])
            .unwrap();
        p2 += number
            .trim()
            .split_whitespace()
            .map(|d| {
                let d: String = d.chars().sorted().collect();
                resolved.iter().position(|c| c == &d).unwrap()
            })
            .fold(0, |acc, d| acc * 10 + d);
    }
    (p1, p2)
}

#[test]
fn t() {
    assert_eq!(
        run(
            " acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf"
        ),
        (0, 5353)
    )
}

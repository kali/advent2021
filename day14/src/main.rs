use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    dbg!(run(input.trim()));
}

#[derive(Clone, Debug)]
struct RunSummary {
    pairs: HashMap<(char, char), usize>,
    items: HashMap<char, usize>,
}

impl RunSummary {
    fn from(input: &str) -> RunSummary {
        let pairs = input.chars().tuple_windows::<(_, _)>().counts();
        let items = input.chars().counts();
        RunSummary { pairs, items }
    }

    fn expand(&self, rules: &HashMap<(char, char), char>) -> RunSummary {
        let mut items = self.items.clone();
        let mut pairs = HashMap::new();
        for (pair, n) in self.pairs.iter() {
            let mid = rules[pair];
            *items.entry(mid).or_default() += n;
            *pairs.entry((pair.0, mid)).or_default() += n;
            *pairs.entry((mid, pair.1)).or_default() += n;
        }
        RunSummary { items, pairs }
    }

    fn score(&self) -> usize {
        let (min, max) = self.items.values().minmax().into_option().unwrap();
        max - min
    }
}

fn run(input: &str) -> (usize, usize) {
    let (start, rules) = input.split_once("\n\n").unwrap();
    let rules: HashMap<(char, char), char> = rules
        .lines()
        .map(|line| {
            (
                (line.chars().nth(0).unwrap(), line.chars().nth(1).unwrap()),
                line.chars().nth(6).unwrap(),
            )
        })
        .collect();
    let start = RunSummary::from(&start);
    let p1 = (0..10)
        .fold(start.clone(), |acc, _| acc.expand(&rules))
        .score();
    let p2 = (0..40).fold(start, |acc, _| acc.expand(&rules)).score();
    (p1, p2)
}

#[test]
fn t() {
    assert_eq!(
        run("NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C"),
        (1588, 2188189693529)
    );
}

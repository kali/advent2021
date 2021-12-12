use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    dbg!(run(input.trim()));
}

fn run(input: &str) -> (usize, usize) {
    let mut links = HashMap::<&str, Vec<&str>>::new();
    for line in input.lines() {
        let (left, right) = line.split_once("-").unwrap();
        links.entry(left).or_default().push(right);
        links.entry(right).or_default().push(left);
    }
    let p1 = pathfinding::directed::bfs::bfs_reach(vec!["start"], |path| {
        if path.last() == Some(&"end") {
            vec![]
        } else {
            links[path.last().unwrap()]
                .iter()
                .filter(|next| next.chars().next().unwrap().is_uppercase() || !path.contains(next))
                .map(|next| {
                    path.iter()
                        .map(|s| *s)
                        .chain(std::iter::once(*next))
                        .collect()
                })
                .collect()
        }
    })
    .filter(|path| path.last() == Some(&"end"))
    .count();
    let p2 = pathfinding::directed::bfs::bfs_reach(vec!["start"], |path| {
        if path.last() == Some(&"end") {
            vec![]
        } else {
            let already_revisited = path
                .iter()
                .filter(|p| p.chars().next().unwrap().is_lowercase())
                .counts()
                .values()
                .any(|count| *count == 2);
            links[path.last().unwrap()]
                .iter()
                .filter(|next| {
                    **next != "start"
                        && (next.chars().next().unwrap().is_uppercase()
                            || !already_revisited
                            || !path.contains(next))
                })
                .map(|next| {
                    path.iter()
                        .map(|s| *s)
                        .chain(std::iter::once(*next))
                        .collect()
                })
                .collect()
        }
    })
    .filter(|path| path.last() == Some(&"end"))
    .count();
    (p1, p2)
}

#[test]
fn t() {
    assert_eq!(
        run("start-A
start-b
A-c
A-b
b-d
A-end
b-end"),
        (10, 36)
    );
}

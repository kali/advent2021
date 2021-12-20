use itertools::Itertools;
use std::{collections::HashSet, ops::RangeInclusive};

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    dbg!(run(input.trim()));
}

struct Image {
    back: bool,
    fore: HashSet<(isize, isize)>,
}

impl Image {
    fn bouding_box(&self) -> (RangeInclusive<isize>, RangeInclusive<isize>) {
        let x = self
            .fore
            .iter()
            .map(|(x, _)| x)
            .minmax()
            .into_option()
            .unwrap();
        let y = self
            .fore
            .iter()
            .map(|(_, y)| y)
            .minmax()
            .into_option()
            .unwrap();
        (*x.0..=*x.1, *y.0..=*y.1)
    }

    fn dump(&self) {
        let (x, y) = self.bouding_box();
        for y in y {
            for x in x.clone() {
                print!(
                    "{}",
                    if self.fore.contains(&(x, y)) ^ self.back {
                        '#'
                    } else {
                        '.'
                    }
                );
            }
            println!()
        }
        println!()
    }

    fn apply(&self, algo: &[bool]) -> Image {
        let mut fore = HashSet::new();
        let back = self.back ^ algo[0];
        let (x, y) = self.bouding_box();
        for x in x.start() - 1..=x.end() + 1 {
            for y in y.start() - 1..=y.end() + 1 {
                let mask = [
                    (-1, -1),
                    (0, -1),
                    (1, -1),
                    (-1, 0),
                    (0, 0),
                    (1, 0),
                    (-1, 1),
                    (0, 1),
                    (1, 1),
                ]
                .iter()
                .map(|(dx, dy)| (self.fore.contains(&(x + dx, y + dy)) ^ self.back) as usize)
                .fold(0, |acc, it| (acc << 1) + it);
                if algo[mask] ^ back {
                    fore.insert((x, y));
                }
            }
        }
        Image { back, fore }
    }
}

fn run(input: &str) -> (usize, usize) {
    let (algo, image) = input.split_once("\n\n").unwrap();
    let algo: Vec<bool> = algo
        .chars()
        .filter(|&c| c == '.' || c == '#')
        .map(|c| c == '#')
        .collect();
    let mut fore = HashSet::new();
    for (y, row) in image.lines().enumerate() {
        for (x, c) in row.chars().enumerate() {
            if c == '#' {
                fore.insert((x as isize, y as isize));
            }
        }
    }
    let image = Image { back: false, fore };
    let p1 = image.apply(&algo).apply(&algo).fore.len();
    let p2 = (0..50).fold(image, |acc, _| acc.apply(&algo)).fore.len();
    (p1, p2)
}

#[test]
fn t() {
    assert_eq!(
        run(
            "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##
#..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###
.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#.
.#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#.....
.#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#..
...####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.....
..##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###"
        ),
        (35, 3351)
    );
}

use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    dbg!(run(input.trim()));
}

type Translation = (isize, isize, isize);

#[derive(Copy, Clone, Debug, PartialEq, Default)]
struct Position {
    rotation: usize,
    translation: Translation,
}

impl Position {
    fn rotate(id: usize, (x, y, z): Translation) -> Translation {
        let (x, y, z) = match id % 6 {
            0 => (x, y, z),
            1 => (y, z, x),
            2 => (z, x, y),
            3 => (x, -y, -z),
            4 => (y, -z, -x),
            5 => (z, -x, -y),
            _ => panic!(),
        };
        match id / 6 {
            0 => (x, y, z),
            1 => (-y, x, z),
            2 => (-x, -y, z),
            3 => (y, -x, z),
            _ => panic!(),
        }
    }

    fn transform(&self, (x, y, z): Translation) -> Translation {
        let (x, y, z) = Self::rotate(self.rotation, (x, y, z));
        (
            x + self.translation.0,
            y + self.translation.1,
            z + self.translation.2,
        )
    }
}

fn parse(input: &str) -> Vec<Vec<(isize, isize, isize)>> {
    input
        .split("\n\n")
        .map(|scan| {
            scan.lines()
                .skip(1)
                .map(|line| {
                    line.split(",")
                        .map(|s| s.parse::<isize>().unwrap())
                        .collect_tuple()
                        .unwrap()
                })
                .collect()
        })
        .collect()
}

fn run(input: &str) -> (usize, usize) {
    let scans = parse(input);
    let mut positions: Vec<Option<Position>> = vec![None; scans.len()];
    positions[0] = Some(Position::default());
    let mut transformed: Vec<Option<Vec<(isize, isize, isize)>>> = vec![None; scans.len()];
    transformed[0] = Some(scans[0].clone());
    let mut tried = std::collections::HashSet::new();
    while positions.iter().any(|p| p.is_none()) {
        let (located, free): (Vec<_>, _) = (0..scans.len()).partition(|&p| positions[p].is_some());
        for (&loc, &free) in located.iter().cartesian_product(free.iter()) {
            if !tried.insert((loc, free)) {
                continue;
            }
            if let Some(pos) = align(&transformed[loc].as_ref().unwrap(), &scans[free]) {
                transformed[free] = Some(scans[free].iter().map(|p| pos.transform(*p)).collect());
                positions[free] = Some(pos);
                eprintln!("located {}", free);
                break;
            }
        }
    }
    let mut pts = std::collections::HashSet::new();
    for scan in transformed {
        for pt in scan.unwrap() {
            pts.insert(pt);
        }
    }
    let p1 = pts.len();
    let p2 = positions
        .iter()
        .tuple_combinations()
        .map(|(a, b)| {
            let (a, b) = (a.unwrap().translation, b.unwrap().translation);
            (a.0 - b.0).abs() + (a.1 - b.1).abs() + (a.2 - b.2).abs()
        })
        .max()
        .unwrap() as usize;
    (p1, p2)
}

fn align(left: &[Translation], right: &[Translation]) -> Option<Position> {
    align_min(left, right, 12)
}

fn align_min(left: &[Translation], right: &[Translation], min: usize) -> Option<Position> {
    for (l, r) in (0..left.len()).cartesian_product(0..right.len()) {
        for rotation in 0..24 {
            let rot_right = Position::rotate(rotation, right[r]);
            let translation = (
                left[l].0 - rot_right.0,
                left[l].1 - rot_right.1,
                left[l].2 - rot_right.2,
            );
            let position = Position {
                rotation,
                translation,
            };
            assert_eq!(position.transform(right[r]), left[l]);
            if right
                .iter()
                .map(|pt| {
                    let dst = position.transform(*pt);
                    dst
                })
                .filter(|pt| left.contains(pt))
                .count()
                >= min
            {
                return Some(position);
            }
        }
    }
    None
}

#[test]
fn t() {
    let input = std::fs::read_to_string("test").unwrap();
    assert_eq!(run(input.trim()), (79, 3621));
}

#[test]
fn align_0_1() {
    let input = std::fs::read_to_string("test").unwrap();
    let scans = parse(input.trim());
    assert_eq!(
        align(&*scans[0], &*scans[1]),
        Some(Position {
            rotation: 15,
            translation: (68, -1246, -43)
        })
    );
}

#[test]
fn align_2pt() {
    let left = vec![(-618, -824, -621), (-537, -823, -458)];
    let right = vec![(686, 422, 578), (605, 423, 415)];
    // left: -81, -1, -163 // right: 81, -1, 163
    for i in 0..24 {
        eprintln!("{:?}", Position::rotate(i, (-81, -1, -163)));
    }
    assert_eq!(
        align_min(&*left, &*right, 2),
        Some(Position {
            rotation: 15,
            translation: (68, -1246, -43)
        })
    );
}

#[test]
fn align_2pt_tr() {
    let left = vec![(1, 0, 0), (0, 1, 0)];
    let right = vec![(1, 0, 10), (0, 1, 10)];
    assert_eq!(
        align_min(&*left, &*right, 2),
        Some(Position {
            rotation: 0,
            translation: (0, 0, -10)
        })
    );
}

#[test]
fn align_3pt_rot() {
    let left = vec![(0, 0, 0), (1, 0, 0), (0, 1, 0)];
    let right = vec![(0, 0, 0), (1, 0, 0), (0, 0, 1)];
    assert_eq!(
        align_min(&*left, &*right, 3),
        Some(Position {
            rotation: 2,
            translation: (0, 0, 0),
        })
    );
}

#[test]
fn align_3pt_rot_tr() {
    let left = vec![(1, 0, 0), (0, 2, 0), (0, 0, 3)];
    let right = vec![(10, 0, 1), (12, 0, 0), (10, 3, 0)];
    assert_eq!(
        align_min(&*left, &*right, 3),
        Some(Position {
            rotation: 2,
            translation: (0, -10, 0),
        })
    );
}

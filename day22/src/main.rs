use itertools::Itertools;
use std::{collections::HashSet, ops::Range};

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    dbg!(run(input.trim(), false));
    dbg!(run(input.trim(), true));
}

fn run(input: &str, part2: bool) -> usize {
    let mut on = HashSet::new();
    let data: Vec<(bool, Range<isize>, Range<isize>, Range<isize>)> = input
        .lines()
        .map(|line| {
            let (switch, xmin, xmax, ymin, ymax, zmin, zmax) = scan_fmt::scan_fmt!(
                line,
                "{} x={}..{},y={}..{},z={}..{}",
                String,
                isize,
                isize,
                isize,
                isize,
                isize,
                isize
            )
            .unwrap();
            (
                switch == "on",
                xmin..xmax + 1,
                ymin..ymax + 1,
                zmin..zmax + 1,
            )
        })
        .filter(|(_, x, y, z)| {
            part2
                || !(x.end < -50
                    || x.start > 50
                    || y.end < -50
                    || y.start > 50
                    || z.end < -50
                    || z.start > 50)
        })
        .collect();
    let xs: Vec<isize> = data
        .iter()
        .flat_map(|(_, x, _, _)| [x.start, x.end].into_iter())
        .sorted()
        .collect();
    let ys: Vec<isize> = data
        .iter()
        .flat_map(|(_, _, y, _)| [y.start, y.end].into_iter())
        .sorted()
        .collect();
    let zs: Vec<isize> = data
        .iter()
        .flat_map(|(_, _, _, z)| [z.start, z.end].into_iter())
        .sorted()
        .collect();
    for (switch, x, y, z) in data {
        let xmin = xs.binary_search(&x.start).unwrap();
        let xmax = xs.binary_search(&x.end).unwrap();
        let ymin = ys.binary_search(&y.start).unwrap();
        let ymax = ys.binary_search(&y.end).unwrap();
        let zmin = zs.binary_search(&z.start).unwrap();
        let zmax = zs.binary_search(&z.end).unwrap();
        for x in xmin..xmax {
            for y in ymin..ymax {
                for z in zmin..zmax {
                    if switch {
                        on.insert((x, y, z));
                    } else {
                        on.remove(&(x, y, z));
                    }
                }
            }
        }
    }
    on.iter()
        .map(|(x, y, z)| (xs[x + 1] - xs[*x]) * (ys[y + 1] - ys[*y]) * (zs[z + 1] - zs[*z]))
        .sum::<isize>() as usize
}

#[test]
fn t() {
    let input = "
on x=-20..26,y=-36..17,z=-47..7
on x=-20..33,y=-21..23,z=-26..28
on x=-22..28,y=-29..23,z=-38..16
on x=-46..7,y=-6..46,z=-50..-1
on x=-49..1,y=-3..46,z=-24..28
on x=2..47,y=-22..22,z=-23..27
on x=-27..23,y=-28..26,z=-21..29
on x=-39..5,y=-6..47,z=-3..44
on x=-30..21,y=-8..43,z=-13..34
on x=-22..26,y=-27..20,z=-29..19
off x=-48..-32,y=26..41,z=-47..-37
on x=-12..35,y=6..50,z=-50..-2
off x=-48..-32,y=-32..-16,z=-15..-5
on x=-18..26,y=-33..15,z=-7..46
off x=-40..-22,y=-38..-28,z=23..41
on x=-16..35,y=-41..10,z=-47..6
off x=-32..-23,y=11..30,z=-14..3
on x=-49..-5,y=-3..45,z=-29..18
off x=18..30,y=-20..-8,z=-3..13
on x=-41..9,y=-7..43,z=-33..15
on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
on x=967..23432,y=45373..81175,z=27513..53682
";
    assert_eq!(run(input.trim(), false), 590784);
}

#[test]
fn t1() {
    let input = "
on x=-20..26,y=-36..17,z=-47..7
";
    assert_eq!(run(input.trim(), false), (20+26+1)*(36+17+1)*(47+7+1)
);
}

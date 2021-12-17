use std::ops::Range;

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    dbg!(run(input.trim()));
}

fn hit(tx: &Range<isize>, ty: &Range<isize>, mut vx: isize, mut vy: isize) -> Option<isize> {
    let mut x = 0;
    let mut y = 0;
    let mut top = 0;
    loop {
        x += vx;
        y += vy;
        top = top.max(y);
        if x >= tx.start && x <= tx.end && y >= ty.start && y <= ty.end {
            return Some(top);
        }
        vx = (vx - 1).max(0);
        vy = vy - 1;
        if x > tx.end || y < ty.start {
            return None;
        }
    }
}

fn run(input: &str) -> (usize, usize) {
    let (x1, x2, y1, y2) = scan_fmt::scan_fmt!(
        input.trim(),
        "target area: x={}..{}, y={}..{}",
        isize,
        isize,
        isize,
        isize
    )
    .unwrap();
    let ref tx = x1.min(x2)..x1.max(x2);
    let ref ty = y1.min(y2)..y1.max(y2);
    let vx_range = 1..tx.end + 1;
    let vy_range = -ty.start.abs()..ty.start.abs() + 1;
    let p1 = vx_range.clone()
        .flat_map(|vx| vy_range.clone().map(move |vy| hit(&tx, &ty, vx, vy)))
        .filter_map(|x| x)
        .max()
        .unwrap();
    let p2 = vx_range
        .flat_map(|vx| vy_range.clone().map(move |vy| hit(&tx, &ty, vx, vy)))
        .filter_map(|x| x)
        .count();
    (p1 as usize, p2 as usize)
}

#[test]
fn t() {
    assert_eq!(run("target area: x=20..30, y=-10..-5"), (45, 112));
}

#[test]
fn t_7_2() {
    assert_eq!(hit(&(20..30), &(-10..-5), 7, 2), Some(3))
}

#[test]
fn t_6_9() {
    assert_eq!(hit(&(20..30), &(-10..-5), 6, 9), Some(45))
}

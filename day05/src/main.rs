fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let lines: Vec<(i64, i64, i64, i64)> = input
        .trim()
        .lines()
        .map(|l| {
            let (from, to) = l.split_once(" -> ").unwrap();
            let (x1, y1) = from.split_once(",").unwrap();
            let (x2, y2) = to.split_once(",").unwrap();
            (
                x1.parse().unwrap(),
                y1.parse().unwrap(),
                x2.parse().unwrap(),
                y2.parse().unwrap(),
            )
        })
        .collect();
    let width = lines.iter().map(|(x1, _, x2, _)| x1.max(x2)).max().unwrap();
    let height = lines.iter().map(|(_, y1, _, y2)| y1.max(y2)).max().unwrap();
    for part in [1, 2] {
        let mut seen = vec![0; (width + 1) as usize * (height + 1) as usize];
        for &(x1, y1, x2, y2) in &lines {
            if x1 == x2 {
                for y in y1.min(y2)..=y1.max(y2) {
                    seen[(x1 + width * y) as usize] += 1
                }
            } else if y1 == y2 {
                for x in x1.min(x2)..=x1.max(x2) {
                    seen[(x + width * y1) as usize] += 1
                }
            } else if part == 2 {
                let stride = ((x2 - x1).signum(), (y2 - y1).signum());
                let mut pos = (x1, y1);
                seen[(x1 + width * y1) as usize] += 1;
                while pos != (x2, y2) {
                    //dbg!(pos);
                    pos = (pos.0 + stride.0, pos.1 + stride.1);
                    seen[(pos.0 + width * pos.1) as usize] += 1;
                }
            }
        }
        let p = seen.iter().filter(|v| **v > 1).count();
        dbg!(p);
    }
}

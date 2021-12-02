fn main() {
    let mut horiz = 0;
    let mut vert = 0;
    for line in std::fs::read_to_string("input").unwrap().trim().lines() {
        let (dir, val) = line.split_once(" ").unwrap();
        let val: i64 = val.parse().unwrap();
        match dir {
            "forward" => horiz += val,
            "down" => vert += val,
            "up" => vert -= val,
            _ => panic!(),
        }
    }
    dbg!(horiz * vert);
    let mut horiz = 0;
    let mut aim = 0;
    let mut vert = 0;
    for line in std::fs::read_to_string("input").unwrap().trim().lines() {
        let (dir, val) = line.split_once(" ").unwrap();
        let val: i64 = val.parse().unwrap();
        match dir {
            "forward" => {
                horiz += val;
                vert += aim * val
            }
            "down" => aim += val,
            "up" => aim -= val,
            _ => panic!(),
        }
    }
    dbg!(horiz * vert);
}

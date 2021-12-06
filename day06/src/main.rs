fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    dbg!(run(&input.trim(), 80));
    dbg!(run(&input.trim(), 256));
}

fn run(input:&str, days: usize) -> usize {
    let mut numbers = vec!(0; 9);
    for n in input.split(",") {
        let n = (n.as_bytes()[0] as u8 - b'0') as usize;
        numbers[n] += 1;
    }
    for _ in 0..days {
        let ripe = numbers.remove(0);
        numbers.push(ripe);
        numbers[6] += ripe;
    }
    numbers.iter().sum::<usize>()
}

#[test]
fn t() {
    assert_eq!(run("3,4,3,1,2", 18), 26);
    assert_eq!(run("3,4,3,1,2", 80), 5934);
}

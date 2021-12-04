fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    dbg!(run(&input));
}

fn run(input: &str) -> (i64, i64) {
    let lines: Vec<String> = input.lines().map(|s| s.to_owned()).collect::<Vec<_>>();
    let mut ones = vec![0; lines[0].len()];
    for line in &lines {
        line.chars()
            .enumerate()
            .for_each(|(ix, v)| ones[ix] += (v == '1') as usize as i64);
    }
    let gamma = ones.iter().fold(0i64, |acc, one| {
        (acc << 1) + (*one >= (lines.len() as i64 / 2)) as i64
    });
    let mask = (1 << ones.len()) - 1;
    let epsilon = !gamma & mask;
    let mut remaining = lines.clone();
    for ix in 0.. {
        let ones = remaining
            .iter()
            .filter(|l| l.chars().nth(ix).unwrap() == '1')
            .count();
        let zeros = remaining.len() - ones;
        let wanted = if ones >= zeros {
            '1'
        } else {
            '0'
        };
        remaining.retain(|line| line.chars().nth(ix).unwrap() == wanted);
        if remaining.len() == 1 {
            break;
        }
    }
    let ox = remaining[0]
        .chars()
        .fold(0i64, |acc, one| (acc << 1) + (one == '1') as i64);
    let mut remaining = lines.clone();
    for ix in 0.. {
        let ones = remaining
            .iter()
            .filter(|l| l.chars().nth(ix).unwrap() == '1')
            .count();
        let zeros = remaining.len() - ones;
        let wanted = if zeros <= ones {
            '0'
        } else {
            '1'
        };
        remaining.retain(|line| line.chars().nth(ix).unwrap() == wanted);
        if remaining.len() == 1 {
            break;
        }
    }
    let co2 = remaining[0]
        .chars()
        .fold(0i64, |acc, one| (acc << 1) + (one == '1') as i64);
    (epsilon * gamma, ox * co2)
}

#[test]
fn t() {
    assert_eq!(
        run("00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"),
        (198, 230)
    );
}

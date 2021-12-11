fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    dbg!(run(input.trim()));
}

fn run(input: &str) -> (usize, usize) {
    let mut p1 = 0;
    let mut autocomplete = vec![];
    'l: for line in input.lines() {
        let mut stack = vec![];
        for c in line.chars() {
            match c {
                '(' => stack.push(c),
                '[' => stack.push(c),
                '{' => stack.push(c),
                '<' => stack.push(c),
                ')' if stack.last().unwrap() != &'(' => {
                    p1 += 3;
                    continue 'l;
                }
                ']' if stack.last().unwrap() != &'[' => {
                    p1 += 57;
                    continue 'l;
                }
                '}' if stack.last().unwrap() != &'{' => {
                    p1 += 1197;
                    continue 'l;
                }
                '>' if stack.last().unwrap() != &'<' => {
                    p1 += 25137;
                    continue 'l;
                }
                _ => {
                    stack.pop();
                }
            }
        }
        let score = stack
            .iter()
            .rev()
            .map(|c| match c {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => panic!(),
            })
            .fold(0, |acc, x| acc * 5 + x);
        if score != 0 {
            autocomplete.push(score);
        }
    }
    autocomplete.sort();
    (p1, autocomplete[autocomplete.len() / 2])
}

#[test]
fn t() {
    assert_eq!(
        run("[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"),
        (26397, 288957)
    );
}

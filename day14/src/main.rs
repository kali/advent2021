fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    dbg!(run(input.trim()));
}

fn run(input: &str) -> usize {
    input.len()
}

#[test]
fn t() {
    assert_eq!(run("input"), 12);
}

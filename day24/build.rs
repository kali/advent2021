use std::io::Write;

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let mut rust = std::fs::File::create(format!("{}/func.rs", out_dir)).unwrap();
    for (ix, block) in input
        .split("inp w")
        .filter(|block| !block.trim().is_empty())
        .enumerate()
    {
        writeln!(rust, "pub fn eval_{}(mut z: isize, w: isize) -> isize {{", ix).unwrap();
        writeln!(rust, "let mut x:isize = 0; let mut y:isize = 0;").unwrap();
        for line in block.trim().lines() {
            let (op, args) = line.split_once(" ").unwrap();
            let args = args.split_whitespace().collect::<Vec<_>>();
            match op {
                "mul" => writeln!(rust, "{} *= {};", args[0], args[1]).unwrap(),
                "add" => writeln!(rust, "{} += {};", args[0], args[1]).unwrap(),
                "mod" => writeln!(rust, "{} %= {};", args[0], args[1]).unwrap(),
                "div" => writeln!(rust, "{} /= {};", args[0], args[1]).unwrap(),
                "eql" => {
                    writeln!(rust, "{} = ({} == {}) as isize;", args[0], args[0], args[1]).unwrap()
                }
                _ => panic!(),
            }
        }
        writeln!(rust, "z }}").unwrap();
    }
}

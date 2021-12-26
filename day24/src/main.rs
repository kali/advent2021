use rayon::prelude::*;

include!(concat!(env!("OUT_DIR"), "/func.rs"));

fn second_half<R: IntoIterator<Item = isize> + Clone + Send + Sync>(
    z: isize,
    range: &R,
) -> Option<isize> {
    let w5_w6_z: Vec<(isize, isize, isize)> = range
        .clone()
        .into_iter()
        .map(|w5| (w5, eval_5(z, w5)))
        .flat_map(move |(w5, z)| {
            range
                .clone()
                .into_iter()
                .map(move |w6| (w5, w6, eval_6(z, w6)))
        })
        .collect();

    w5_w6_z
        .into_par_iter()
        .map(|(w5, w6, z)| {
            for w7 in range.clone() {
                let z = eval_7(z, w7);
                for w8 in range.clone() {
                    let z = eval_8(z, w8);
                    for w9 in range.clone() {
                        let z = eval_9(z, w9);
                        for w10 in range.clone() {
                            let z = eval_10(z, w10);
                            for w11 in range.clone() {
                                let z = eval_11(z, w11);
                                for w12 in range.clone() {
                                    let z = eval_12(z, w12);
                                    for w13 in range.clone() {
                                        let z = eval_13(z, w13);
                                        if z == 0 {
                                            let tail = [w5, w6, w7, w8, w9, w10, w11, w12, w13]
                                                .into_iter()
                                                .fold(0, |acc, it| acc * 10 + it);
                                            return Some(tail);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            None
        })
        .find_first(|r| r.is_some())
        .map(|r| r.unwrap())
}

fn search<R: IntoIterator<Item = isize> + Clone + Send + Sync>(range: &R) -> isize {
    let mut tried = std::collections::HashSet::new();
    let mut skipped = 0;
    for w0 in range.clone() {
        let z = eval_0(0, w0);
        for w1 in range.clone() {
            let z = eval_1(z, w1);
            for w2 in range.clone() {
                let z = eval_2(z, w2);
                for w3 in range.clone() {
                    let z = eval_3(z, w3);
                    for w4 in range.clone() {
                        let z = eval_4(z, w4);
                        if tried.insert(z) {
                            if let Some(ok) = second_half(z, range) {
                                let result = (10_000 * w0 + 1000 * w1 + 100 * w2 + 10 * w3 + w4)
                                    * 1_000_000_000
                                    + ok;
                                return result;
                            }
                        } else {
                            skipped += 1;
                        }
                    }
                }
            }
            eprintln!(
                "{}{} (tried z:{} skipped:{})",
                w0,
                w1,
                tried.len(),
                skipped
            );
        }
    }
    unreachable!();
}

fn main() {
    dbg!(search(&(1..10).rev()));
    dbg!(search(&(1..10)));
}

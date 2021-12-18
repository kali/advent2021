use nom::bytes::complete::tag;
use nom::combinator::{map, map_res};
use nom::sequence::{delimited, separated_pair};
use nom::{branch::alt, character::complete::digit1, IResult};

#[derive(Clone, PartialEq)]
struct Pair(Item, Item);

impl std::fmt::Debug for Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:?},{:?}]", self.0, self.1)
    }
}

impl Pair {
    fn lits(a: usize, b: usize) -> Pair {
        Pair(Item::Literal(a), Item::Literal(b))
    }

    fn reduce(&self) -> Pair {
        let mut result = self.clone();
        loop {
            if let Some(ex) = result.explode_step(0) {
                result = ex.1;
                continue;
            }
            if let Some(split) = result.split_step() {
                result = split;
                continue;
            }
            break;
        }
        return result;
    }

    fn explode_step(&self, depth: usize) -> Option<(Option<usize>, Pair, Option<usize>)> {
        if let Some((left, mid, right)) = self.0.explode_step(depth + 1) {
            if let Some(right) = right {
                let right = self.1.inject_from_left(right);
                Some((left, Pair(mid, right), None))
            } else {
                Some((left, Pair(mid, self.1.clone()), None))
            }
        } else if let Some((left, mid, right)) = self.1.explode_step(depth + 1) {
            if let Some(left) = left {
                let left = self.0.inject_from_right(left);
                Some((None, Pair(left, mid), right))
            } else {
                Some((None, Pair(self.0.clone(), mid), right))
            }
        } else {
            None
        }
    }

    fn split_step(&self) -> Option<Pair> {
        if let Some(left) = self.0.split_step() {
            Some(Pair(left, self.1.clone()))
        } else if let Some(right) = self.1.split_step() {
            Some(Pair(self.0.clone(), right))
        } else {
            None
        }
    }

    fn magnitude(&self) -> usize {
        self.0.magnitude() * 3 + self.1.magnitude() * 2
    }
}

#[derive(Clone, PartialEq)]
enum Item {
    Literal(usize),
    Pair(Box<Pair>),
}

impl std::fmt::Debug for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Item::Literal(n) => write!(f, "{}", n),
            Item::Pair(p) => write!(f, "{:?}", p),
        }
    }
}

impl From<Pair> for Item {
    fn from(p: Pair) -> Self {
        Item::Pair(Box::new(p))
    }
}

impl From<usize> for Item {
    fn from(n: usize) -> Self {
        Item::Literal(n)
    }
}

impl Item {
    fn explode_step(&self, depth: usize) -> Option<(Option<usize>, Item, Option<usize>)> {
        match self {
            Item::Literal(_) => None,
            Item::Pair(p) if depth < 4 => p.explode_step(depth).map(|(l, m, r)| (l, m.into(), r)),
            Item::Pair(p) => Some((p.0.as_lit(), 0.into(), p.1.as_lit())),
        }
    }

    fn split_step(&self) -> Option<Item> {
        match self {
            Item::Literal(n) if *n < 10 => None,
            Item::Literal(n) => Some(Pair::lits(*n / 2, *n - *n / 2).into()),
            Item::Pair(p) => p.split_step().map(|p| p.into()),
        }
    }

    fn as_lit(&self) -> Option<usize> {
        if let Item::Literal(n) = self {
            Some(*n)
        } else {
            None
        }
    }

    fn inject_from_left(&self, n: usize) -> Item {
        match self {
            Item::Literal(l) => Item::Literal(*l + n),
            Item::Pair(p) => Item::Pair(Box::new(Pair(p.0.inject_from_left(n), p.1.clone()))),
        }
    }

    fn inject_from_right(&self, n: usize) -> Item {
        match self {
            Item::Literal(l) => Item::Literal(*l + n),
            Item::Pair(p) => Item::Pair(Box::new(Pair(p.0.clone(), p.1.inject_from_right(n)))),
        }
    }

    fn magnitude(&self) -> usize {
        match self {
            Item::Literal(n) => *n,
            Item::Pair(p) => p.magnitude(),
        }
    }
}

impl std::ops::Add<Pair> for Pair {
    type Output = Pair;
    fn add(self, rhs: Pair) -> Self::Output {
        Pair(Item::Pair(Box::new(self)), Item::Pair(Box::new(rhs))).reduce()
    }
}

fn item(i: &str) -> IResult<&str, Item> {
    alt((
        map_res(digit1, |s: &str| s.parse().map(Item::Literal)),
        map(pair, |p| Item::Pair(Box::new(p))),
    ))(i)
}

fn pair(i: &str) -> IResult<&str, Pair> {
    map(
        delimited(tag("["), separated_pair(item, tag(","), item), tag("]")),
        |(a, b)| Pair(a, b),
    )(i)
}

fn parse(i: &str) -> Pair {
    pair(i).unwrap().1
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    dbg!(run(input.trim()));
}

fn run(input: &str) -> (usize, usize) {
    let p1 = input
        .lines()
        .map(parse)
        .reduce(|acc, n| acc + n)
        .unwrap()
        .magnitude();
    let p2 = input
        .lines()
        .map(parse)
        .permutations(2)
        .map(|v| (v[0].clone() + v[1].clone()).magnitude())
        .max()
        .unwrap();
    (p1, p2)
}

#[test]
fn t1() {
    assert_eq!(
        parse("[[[[4,3],4],4],[7,[[8,4],9]]]") + parse("[1,1]"),
        parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")
    )
}

#[test]
fn explode() {
    assert_eq!(
        parse("[[[[[9,8],1],2],3],4]").explode_step(0).unwrap().1,
        parse("[[[[0,9],2],3],4]")
    )
}

#[test]
fn t() {
    assert_eq!(
        run("[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]"),
        (4140, 3993)
    );
}

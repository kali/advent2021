#![allow(non_snake_case)]

use nom::bits::streaming::take;
use nom::multi::many_m_n;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug, Clone, PartialEq)]
enum Packet {
    Operator(usize, usize, Vec<Packet>),
    Litteral(usize, usize),
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    dbg!(run(input.trim()));
}

fn to_bits(input: &str) -> Vec<u8> {
    input
        .as_bytes()
        .chunks(2)
        .map(|pair| {
            let s = std::str::from_utf8(pair).unwrap();
            let b = u8::from_str_radix(s, 16).unwrap();
            b
        })
        .collect()
}

fn run(input: &str) -> (usize, usize) {
    let bin = to_bits(input);
    let packet = parse_packet((&*bin, 0)).unwrap().1;
    (sum_of_versions(&packet), eval(&packet))
}

fn sum_of_versions(p: &Packet) -> usize {
    match p {
        Packet::Litteral(v, _) => *v,
        Packet::Operator(v, _, packets) => *v + packets.iter().map(sum_of_versions).sum::<usize>(),
    }
}

fn eval(p: &Packet) -> usize {
    match p {
        Packet::Litteral(_, v) => *v,
        Packet::Operator(_, op, packets) => match op {
            0 => packets.iter().map(eval).sum::<usize>(),
            1 => packets.iter().map(eval).product::<usize>(),
            2 => packets.iter().map(eval).min().unwrap(),
            3 => packets.iter().map(eval).max().unwrap(),
            5 => (eval(&packets[0]) > eval(&packets[1])) as usize,
            6 => (eval(&packets[0]) < eval(&packets[1])) as usize,
            7 => (eval(&packets[0]) == eval(&packets[1])) as usize,
            _ => panic!("{}", op),
        },
    }
}

fn parse_header(input: (&[u8], usize)) -> IResult<(&[u8], usize), (i32, i32)> {
    tuple((take(3usize), take(3usize)))(input)
}

fn parse_packet(i: (&[u8], usize)) -> IResult<(&[u8], usize), Packet> {
    let (i, (version, typeid)) = parse_header(i)?;
    match typeid {
        4 => {
            let (i, lit) = parse_litteral(i)?;
            Ok((i, Packet::Litteral(version as _, lit)))
        }
        op => {
            assert!(op != 4);
            let (i, operands) = parse_operands(i)?;
            Ok((i, Packet::Operator(version as _, op as _, operands)))
        }
    }
}

fn parse_litteral(mut input: (&[u8], usize)) -> IResult<(&[u8], usize), usize> {
    let mut r = 0usize;
    loop {
        let (i, more): (_, usize) = take(1usize)(input)?;
        let (i, n): (_, usize) = take(4usize)(i)?;
        r = (r << 4) + n;
        if more == 0 {
            return Ok((i, r));
        }
        input = i;
    }
}

fn parse_operands(input: (&[u8], usize)) -> IResult<(&[u8], usize), Vec<Packet>> {
    let (i, length_type_id): (_, usize) = take(1usize)(input)?;
    if length_type_id == 0 {
        let (mut i, len) = take(15usize)(i)?;
        let mut packets = vec![];
        let start = i;
        loop {
            let (i2, packet) = parse_packet(i)?;
            packets.push(packet);
            let used = (i2.0.as_ptr() as usize - start.0.as_ptr() as usize) * 8 + i2.1 - start.1;
            if used == len {
                return Ok((i2, packets));
            }
            i = i2;
        }
    } else {
        let (i, len) = take(11usize)(i)?;
        many_m_n(len, len, parse_packet)(i)
    }
}

#[test]
fn d2fe28() {
    let bits = to_bits("D2FE28");
    assert_eq!(
        parse_packet((&bits, 0)).unwrap().1,
        Packet::Litteral(6, 2021)
    );
}

#[test]
fn _38006F45291200() {
    let bits = to_bits("38006F45291200");
    assert_eq!(
        parse_packet((&bits, 0)).unwrap().1,
        Packet::Operator(1, 6, vec!(Packet::Litteral(6, 10), Packet::Litteral(2, 20)))
    );
}

#[test]
fn EE00D40C823060() {
    let bits = to_bits("EE00D40C823060");
    assert_eq!(
        parse_packet((&bits, 0)).unwrap().1,
        Packet::Operator(
            7,
            3,
            vec!(
                Packet::Litteral(2, 1),
                Packet::Litteral(4, 2),
                Packet::Litteral(1, 3)
            )
        )
    );
}

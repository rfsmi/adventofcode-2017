use std::ops::Add;

use nom::{
    IResult,
    bytes::complete::tag,
    character::complete::{digit1, multispace0, space0},
    combinator::map_res,
    multi::many0,
    sequence::{pair, preceded},
};

fn parse(s: &str) -> IResult<&str, Vec<(i32, i32)>> {
    let num = || preceded(space0, map_res(digit1, str::parse));
    let line = pair(num(), preceded(tag(":"), num()));
    many0(preceded(multispace0, line))(s)
}

fn scanner_pos(r: i32, t: i32) -> i32 {
    let r = r - 1;
    r - (t % (r * 2) - r).abs()
}

fn severity(layers: &[(i32, i32)], t: i32) -> Option<i32> {
    layers
        .into_iter()
        .filter(|&&(d, r)| scanner_pos(r, d + t) == 0)
        .map(|&(d, r)| d * r)
        .reduce(Add::add)
}

pub fn solve(s: &str) -> i32 {
    let layers = parse(s).unwrap().1;
    severity(&layers, 0).unwrap()
}

pub fn solve_2(s: &str) -> i32 {
    let layers = parse(s).unwrap().1;
    (0..).find(|&t| severity(&layers, t).is_none()).unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &'static str = "
        0: 3
        1: 2
        4: 4
        6: 4";

    #[test]
    fn test_sample() {
        assert_eq!(solve(SAMPLE), 24);
    }

    #[test]
    fn test_sample_2() {
        assert_eq!(solve_2(SAMPLE), 10)
    }
}

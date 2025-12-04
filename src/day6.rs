use std::{
    collections::HashSet,
    iter::{repeat, zip},
};

use itertools::chain;
use nom::{
    IResult,
    character::complete::{digit1, multispace0},
    combinator::map_res,
    multi::many0,
    sequence::preceded,
};

fn parse(s: &str) -> IResult<&str, Vec<usize>> {
    let int = map_res(digit1, str::parse);
    many0(preceded(multispace0, int))(s)
}

fn run(mut banks: Vec<usize>) -> (Vec<usize>, usize) {
    let mut seen = HashSet::new();
    while seen.insert(banks.clone()) {
        let i = (0..banks.len()).rev().max_by_key(|&i| banks[i]).unwrap();
        let base = banks[i] / banks.len();
        let n_extra = banks[i] % banks.len();
        banks[i] = 0;
        for (i, delta) in zip(
            (0..banks.len()).cycle().skip(i + 1),
            chain!(
                repeat(base + 1).take(n_extra),
                repeat(base).take(banks.len() - n_extra),
            ),
        ) {
            banks[i] += delta;
        }
    }
    (banks, seen.len())
}

pub fn solve(s: &str) -> usize {
    run(parse(s).unwrap().1).1
}

pub fn solve_2(s: &str) -> usize {
    let (banks, _) = run(parse(s).unwrap().1);
    run(banks).1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sample() {
        let sample = "0 2 7 0";
        assert_eq!(solve(sample), 5);
    }
}

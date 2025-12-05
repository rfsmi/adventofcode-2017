use std::ops::BitXor;

use itertools::Itertools;
use nom::{
    IResult,
    bytes::complete::tag,
    character::complete::{digit1, multispace0},
    combinator::map_res,
    multi::separated_list0,
    sequence::preceded,
};

fn parse(s: &str) -> IResult<&str, Vec<i64>> {
    let num = map_res(digit1, str::parse);
    separated_list0(tag(","), preceded(multispace0, num))(s)
}

fn value_at<const N: i64>(mut i: i64, flips: &[(i64, i64)]) -> i64 {
    for &(l, r) in flips.into_iter().rev() {
        if l == r {
            continue;
        }
        let (l, r) = (l.rem_euclid(N), r.rem_euclid(N));
        if l < r && (l..r).contains(&i) || r <= l && !(r..l).contains(&i) {
            i = (l + r - i - 1).rem_euclid(N);
        }
    }
    i
}

fn get_flips(lengths: &[i64]) -> Vec<(i64, i64)> {
    lengths
        .into_iter()
        .scan((0, 0), |(i, s), l| {
            let range = (*i, *i + l);
            *i += l + *s;
            *s += 1;
            Some(range)
        })
        .collect()
}

pub fn solve(s: &str) -> i64 {
    let lengths = parse(s).unwrap().1;
    let flips = get_flips(&lengths);
    value_at::<256>(0, &flips) * value_at::<256>(1, &flips)
}

fn to_hex(nums: impl Iterator<Item = i64>) -> String {
    nums.chunks(16)
        .into_iter()
        .filter_map(|c| c.reduce(BitXor::bitxor))
        .map(|b| format!("{b:02x}"))
        .collect()
}

pub fn solve_2(s: &str) -> String {
    let mut lengths: Vec<_> = s.trim().bytes().map(|b| b as i64).collect();
    lengths.extend([17, 31, 73, 47, 23]);
    lengths = lengths.repeat(64);
    let flips = get_flips(&lengths);
    to_hex((0..256).map(|i| value_at::<256>(i, &flips)))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_value_at() {
        assert_eq!(value_at::<256>(0, &[(254, 1)]), 254);
    }
}

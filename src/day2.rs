use std::str::FromStr;

use itertools::{Itertools, MinMaxResult::MinMax};
use nom::{
    character::complete::{digit1, multispace0, space0},
    combinator::map_res,
    multi::{many0, many1},
    sequence::preceded,
    IResult,
};

fn parse(s: &str) -> IResult<&str, Vec<Vec<u32>>> {
    let num = map_res(digit1, <u32 as FromStr>::from_str);
    let row = many1(preceded(space0, num));
    many0(preceded(multispace0, row))(s)
}

pub fn solve(s: &str) -> u32 {
    let table = parse(s).unwrap().1;
    table
        .into_iter()
        .filter_map(|row| match row.into_iter().minmax() {
            MinMax(a, b) => Some(b - a),
            _ => None,
        })
        .sum()
}

pub fn solve_2(s: &str) -> u32 {
    fn div(row: impl Itertools<Item = u32> + Clone) -> Option<u32> {
        row.into_iter()
            .tuple_combinations()
            .filter(|(a, b)| a % b == 0)
            .map(|(a, b)| a / b)
            .next()
    }
    let table = parse(s).unwrap().1;
    table
        .into_iter()
        .map(|row| {
            None.or(div(row.iter().copied()))
                .or(div(row.iter().copied().rev()))
                .unwrap()
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(
            solve(
                "5 1 9 5
                7 5 3
                2 4 6 8",
            ),
            18
        )
    }

    #[test]
    fn test_sample_2() {
        assert_eq!(
            solve_2(
                "5 9 2 8
                9 4 7 3
                3 8 6 5",
            ),
            9
        )
    }
}

use itertools::Itertools;
use nom::{
    character::complete::{anychar, multispace0},
    combinator::map_opt,
    multi::many0,
    sequence::preceded,
    IResult,
};

fn parse(s: &str) -> IResult<&str, Vec<u32>> {
    let digit = map_opt(anychar, |c| c.to_digit(10));
    many0(preceded(multispace0, digit))(s)
}

pub fn solve(s: &str) -> u32 {
    let nums = parse(s).unwrap().1;
    nums.into_iter()
        .circular_tuple_windows()
        .filter(|(a, b)| a == b)
        .map(|(a, _)| a)
        .sum()
}

pub fn solve_2(s: &str) -> u32 {
    let nums = parse(s).unwrap().1;
    (0..nums.len())
        .filter(|&i| nums[i] == nums[(i + nums.len() / 2) % nums.len()])
        .map(|i| nums[i])
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sample() {
        for (s, r) in [("91212129", 9), ("1234", 0), ("1111", 4), ("1122", 3)] {
            assert_eq!(solve(s), r);
        }
    }

    #[test]
    fn test_sample_2() {
        assert_eq!(solve_2("1212"), 6);
        assert_eq!(solve_2("123123"), 12);
    }
}

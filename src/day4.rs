use itertools::Itertools;
use nom::{
    IResult,
    character::complete::{alpha1, multispace0, space0},
    multi::{many0, many1},
    sequence::preceded,
};

fn parse(s: &str) -> IResult<&str, Vec<Vec<&str>>> {
    let word = preceded(space0, alpha1);
    let line = preceded(multispace0, many1(word));
    many0(line)(s)
}

pub fn solve(s: &str) -> usize {
    let lines = parse(s).unwrap().1;
    lines
        .into_iter()
        .filter(|l| l.iter().unique().count() == l.len())
        .count()
}

pub fn solve_2(s: &str) -> usize {
    let lines = parse(s).unwrap().1;
    lines
        .into_iter()
        .map(|l| {
            l.into_iter()
                .map(|s| s.chars().sorted().collect::<String>())
                .collect_vec()
        })
        .filter(|l| l.iter().unique().count() == l.len())
        .count()
}

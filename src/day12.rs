use std::collections::HashSet;

use nom::{
    IResult,
    bytes::complete::tag,
    character::complete::{digit1, multispace0, space0},
    combinator::map_res,
    multi::{many0, separated_list1},
    sequence::{pair, preceded},
};

fn parse(s: &str) -> IResult<&str, Vec<Vec<usize>>> {
    let num = preceded(space0, map_res(digit1, str::parse));
    let line = preceded(pair(digit1, tag(" <->")), separated_list1(tag(","), num));
    many0(preceded(multispace0, line))(s)
}

fn group(i: usize, edges: &[Vec<usize>]) -> HashSet<usize> {
    let mut seen = HashSet::new();
    let mut stack = vec![i];
    while let Some(i) = stack.pop() {
        if seen.insert(i) {
            stack.extend(edges[i].iter().copied());
        }
    }
    seen
}

pub fn solve(s: &str) -> usize {
    let edges = parse(s).unwrap().1;
    group(0, &edges).len()
}

pub fn solve_2(s: &str) -> usize {
    let edges = parse(s).unwrap().1;
    let mut todo: HashSet<_> = (0..edges.len()).collect();
    for n in 0.. {
        let Some(&i) = todo.iter().next() else {
            return n;
        };
        todo = &todo - &group(i, &edges)
    }
    panic!()
}

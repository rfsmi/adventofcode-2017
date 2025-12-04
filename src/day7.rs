use std::collections::{HashMap, HashSet};

use itertools::{Itertools, MinMaxResult::MinMax};
use nom::{
    IResult,
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1, multispace0, space0},
    combinator::{cut, map, map_res, success},
    multi::{many0, separated_list1},
    sequence::{delimited, preceded, tuple},
};

struct Disc<'a> {
    name: &'a str,
    weight: i32,
    children: Vec<&'a str>,
}

fn parse(s: &str) -> IResult<&str, Vec<Disc<'_>>> {
    let num = preceded(space0, map_res(digit1, str::parse));
    let weight = preceded(space0, delimited(tag("("), num, tag(")")));
    let child = preceded(space0, alpha1);
    let children = preceded(
        space0,
        alt((
            preceded(tag("->"), cut(separated_list1(tag(","), child))),
            success(Vec::new()),
        )),
    );
    let disc = map(
        tuple((alpha1, weight, children)),
        |(name, weight, children)| Disc {
            name,
            weight,
            children,
        },
    );
    many0(preceded(multispace0, disc))(s)
}

pub fn solve(s: &str) -> &str {
    let discs = parse(s).unwrap().1;
    let parents: HashSet<_> = discs.iter().map(|d| d.name).collect();
    let children: HashSet<_> = discs.into_iter().flat_map(|d| d.children).collect();
    parents.difference(&children).next().unwrap()
}

pub fn solve_2(s: &str) -> i32 {
    let root = solve(s);
    let discs = parse(s).unwrap().1;
    let discs: HashMap<_, _> = discs.into_iter().map(|d| (d.name, d)).collect();
    let mut stack = vec![(false, root)];
    let mut retvals: Vec<i32> = Vec::new();
    while let Some((done, n)) = stack.pop() {
        let disc = discs.get(n).unwrap();
        if !done {
            stack.push((true, n));
            stack.extend(disc.children.iter().rev().map(|&n| (false, n)));
            continue;
        }
        let ws = retvals.split_off(retvals.len() - disc.children.len());
        if ws.iter().all_equal() {
            retvals.push(ws.iter().sum::<i32>() + disc.weight);
            continue;
        }
        // We've found our imbalance
        assert!(ws.len() > 2); // Don't bother with this case
        let others_all_equal =
            |&i: &usize| (0..i).chain(i + 1..ws.len()).map(|j| ws[j]).all_equal();
        let i = (0..ws.len()).find(others_all_equal).unwrap();
        let missing_weight = ws[(i + 1) % ws.len()] - ws[i];
        return discs.get(disc.children[i]).unwrap().weight + missing_weight;
    }
    panic!()
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &'static str = "
        pbga (66)
        xhth (57)
        ebii (61)
        havc (66)
        ktlj (57)
        fwft (72) -> ktlj, cntj, xhth
        qoyq (66)
        padx (45) -> pbga, havc, qoyq
        tknk (41) -> ugml, padx, fwft
        jptl (61)
        ugml (68) -> gyxo, ebii, jptl
        gyxo (61)
        cntj (57)";

    #[test]
    fn test_sample() {
        assert_eq!(solve(SAMPLE), "tknk");
    }

    #[test]
    fn test_sample_2() {
        assert_eq!(solve_2(SAMPLE), 60);
    }
}

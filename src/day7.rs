use std::collections::{HashMap, HashSet};

use itertools::Itertools;
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
    #[derive(PartialEq, Eq)]
    struct Subtree {
        balanced: bool,
        weight: i32,
    }

    let root = solve(s);
    let discs = parse(s).unwrap().1;
    let discs: HashMap<_, _> = discs.into_iter().map(|d| (d.name, d)).collect();
    let mut subtrees = HashMap::new();

    // Compute the weight of each subtree and whether it's balanced.
    enum Instr {
        Recurse,
        Record,
    }
    let mut stack = vec![(Instr::Recurse, root)];
    let mut ret: Vec<Subtree> = Vec::new();
    while let Some((instr, n)) = stack.pop() {
        let disc = discs.get(n).unwrap();
        match instr {
            Instr::Recurse => {
                stack.push((Instr::Record, n));
                stack.extend(disc.children.iter().map(|&n| (Instr::Recurse, n)));
            }
            Instr::Record => {
                let rets = ret.split_off(ret.len() - disc.children.len());
                let balanced = rets.iter().all_equal();
                let weight = rets.iter().map(|r| r.weight).sum::<i32>() + disc.weight;
                ret.push(Subtree { balanced, weight });
                subtrees.insert(n, Subtree { balanced, weight });
            }
        }
    }

    // Find the deepest imbalanced node.
    let mut n = root;
    let mut sibling_weight = None;
    loop {
        let children = &discs.get(n).unwrap().children;
        let Some(c) = children.iter().find(|&c| {
            let subtree = subtrees.get(c).unwrap();
            !subtree.balanced
        }) else {
            break;
        };
        n = c;
        sibling_weight = children.iter().find_map(|&c| {
            let subtree = subtrees.get(c).unwrap();
            subtree.balanced.then_some(subtree.weight)
        });
    }

    // Determine which of its children need fixing.
    // Note: fails if only the root node is unbalanced (as in the example).
    let missing_weight = sibling_weight.unwrap() - subtrees.get(n).unwrap().weight;
    let children = &discs.get(n).unwrap().children;
    let c = if missing_weight > 0 {
        (0..children.len()).min_by_key(|&i| subtrees.get(children[i]).unwrap().weight)
    } else {
        (0..children.len()).max_by_key(|&i| subtrees.get(children[i]).unwrap().weight)
    };
    let c = children[c.unwrap()];
    discs.get(c).unwrap().weight + missing_weight
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
}

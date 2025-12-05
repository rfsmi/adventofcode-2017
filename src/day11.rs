use nom::{
    IResult, branch::alt, bytes::complete::tag, character::complete::multispace0,
    combinator::value, multi::separated_list0, sequence::preceded,
};

fn parse(s: &str) -> IResult<&str, Vec<(i32, i32)>> {
    let dir = alt((
        value((1, -1), tag("ne")),
        value((1, 0), tag("se")),
        value((-1, 1), tag("sw")),
        value((-1, 0), tag("nw")),
        value((0, -1), tag("n")),
        value((0, 1), tag("s")),
    ));
    separated_list0(tag(","), preceded(multispace0, dir))(s)
}

fn walk(steps: &[(i32, i32)]) -> impl Iterator<Item = (i32, i32)> + '_ {
    steps.into_iter().scan((0, 0), |(q, r), (dq, dr)| {
        (*q, *r) = (*q + dq, *r + dr);
        Some((*q, *r))
    })
}

fn dist((q, r): (i32, i32)) -> i32 {
    (q.abs() + r.abs() + (-q - r).abs()) / 2
}

pub fn solve(s: &str) -> i32 {
    let steps = parse(s).unwrap().1;
    dist(walk(&steps).last().unwrap())
}

pub fn solve_2(s: &str) -> i32 {
    let steps = parse(s).unwrap().1;
    walk(&steps).map(dist).max().unwrap()
}

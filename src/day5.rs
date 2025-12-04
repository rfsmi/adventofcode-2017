use nom::{
    IResult,
    bytes::complete::tag,
    character::complete::{digit1, multispace0},
    combinator::{map_res, opt, recognize},
    multi::many0,
    sequence::{pair, preceded},
};

fn parse(s: &str) -> IResult<&str, Vec<i32>> {
    let num = map_res(recognize(pair(opt(tag("-")), digit1)), str::parse);
    many0(preceded(multispace0, num))(s)
}

pub fn solve(s: &str) -> usize {
    let mut nums = parse(s).unwrap().1;
    let mut i: i32 = 0;
    let mut count = 0;
    while (0..nums.len() as i32).contains(&i) {
        let offset = &mut nums[i as usize];
        i += *offset;
        *offset += 1;
        count += 1;
    }
    count
}

pub fn solve_2(s: &str) -> usize {
    let mut nums = parse(s).unwrap().1;
    let mut i: i32 = 0;
    let mut count = 0;
    while (0..nums.len() as i32).contains(&i) {
        let offset = &mut nums[i as usize];
        i += *offset;
        *offset += match *offset {
            3.. => -1,
            ..3 => 1,
        };
        count += 1;
    }
    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sample() {
        let sample = "
            0
            3
            0
            1
            -3";
        assert_eq!(solve(sample), 5);
    }
}

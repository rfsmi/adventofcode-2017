use std::collections::HashMap;

use itertools::Itertools;

pub fn run(s: &str) -> (i32, i32) {
    let mut regs: HashMap<&str, i32> = HashMap::new();
    let mut max = 0;
    for l in s.trim().lines() {
        let mut parts = l.split_ascii_whitespace();
        let (dst, op, operand, _, cnd_src, cnd_op, cnd_operand) = parts.next_tuple().unwrap();
        let operand: i32 = operand.parse().unwrap();
        let cnd_operand: i32 = cnd_operand.parse().unwrap();
        let cnd_src = *regs.get(cnd_src).unwrap_or(&0);
        let cnd = match cnd_op {
            "<" => cnd_src < cnd_operand,
            ">" => cnd_src > cnd_operand,
            "<=" => cnd_src <= cnd_operand,
            ">=" => cnd_src >= cnd_operand,
            "==" => cnd_src == cnd_operand,
            "!=" => cnd_src != cnd_operand,
            _ => panic!("unknown operand {cnd_op}"),
        };
        if !cnd {
            continue;
        }
        let operand = match op {
            "inc" => operand,
            "dec" => -operand,
            _ => panic!("unknown operand {op}"),
        };
        let entry = regs.entry(dst).or_default();
        *entry += operand;
        max = max.max(*entry);
    }
    (regs.into_values().max().unwrap(), max)
}

pub fn solve(s: &str) -> i32 {
    run(s).0
}

pub fn solve_2(s: &str) -> i32 {
    run(s).1
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &'static str = "
        b inc 5 if a > 1
        a inc 1 if b < 5
        c dec -10 if a >= 1
        c inc -20 if c == 10";

    #[test]
    fn test_sample() {
        assert_eq!(solve(SAMPLE), 1);
    }

    #[test]
    fn test_sample_2() {
        assert_eq!(solve_2(SAMPLE), 10);
    }
}

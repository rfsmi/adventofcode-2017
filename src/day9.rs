struct Score {
    group: u32,
    garbage: u32,
}

fn next_garbage(s: &mut impl Iterator<Item = char>, score: &mut Score) {
    loop {
        match s.next().expect("unterminated garbage") {
            '!' => {
                s.next().expect("hanging !");
            }
            '>' => break,
            _ => score.garbage += 1,
        }
    }
}

fn next_group(s: &mut impl Iterator<Item = char>, score: &mut Score, depth: u32) {
    score.group += depth;
    let mut has_comma = false;
    loop {
        match s.next().expect("unterminated group") {
            '<' => next_garbage(s, score),
            '{' => next_group(s, score, depth + 1),
            '}' if !has_comma => break,
            c => panic!("expected {{ or <; got {c}"),
        }
        match s.next().expect("unterminated group") {
            '}' => break,
            ',' => has_comma = true,
            c => panic!("expected }} or ,; got {c}"),
        }
    }
}

fn run(s: &str) -> Score {
    let mut score = Score {
        group: 0,
        garbage: 0,
    };
    let mut s = s.trim().chars();
    assert_eq!(s.next(), Some('{'));
    next_group(&mut s, &mut score, 1);
    score
}

pub fn solve(s: &str) -> u32 {
    run(s).group
}

pub fn solve_2(s: &str) -> u32 {
    run(s).garbage
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_samples() {
        assert_eq!(solve("{{{}}}"), 6);
    }
}

use std::{
    collections::HashMap,
    iter::{repeat, zip},
};

use itertools::Itertools;

fn spiral() -> impl Iterator<Item = (i32, i32)> {
    zip([0, -1, 0, 1], [1, 0, -1, 0])
        .cycle()
        .enumerate()
        .flat_map(|(i, (dy, dx))| repeat((dy, dx)).take(1 + i / 2))
        .scan((0, 0), |(y, x), (dy, dx)| {
            let result = (*y, *x);
            (*y, *x) = (*y + dy, *x + dx);
            Some(result)
        })
}

pub fn solve(s: &str) -> i32 {
    let n = s.parse().unwrap();
    for (i, (y, x)) in zip(1.., spiral()) {
        if i == n {
            return y.abs() + x.abs();
        }
    }
    panic!()
}

pub fn solve_2(s: &str) -> u32 {
    let n: u32 = s.parse().unwrap();
    let mut hm: HashMap<_, _> = [((0, 0), 1)].into();
    for (y, x) in spiral().skip(1) {
        let total = (y - 1..y + 2)
            .cartesian_product(x - 1..x + 2)
            .filter_map(|p| hm.get(&p))
            .sum();
        hm.insert((y, x), total);
        if total > n {
            return total;
        }
    }
    todo!()
}

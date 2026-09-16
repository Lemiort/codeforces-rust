// 1084B: Kvass and the Fair Nut
// Problem: https://codeforces.com/problemset/problem/1084/B
// Rating: 1200
// Tags: greedy, implementation
//
#![allow(dead_code)]

use std::io::{self, Read};

struct Scanner<'a> {
    tokens: std::str::SplitWhitespace<'a>,
}

impl<'a> Scanner<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            tokens: input.split_whitespace(),
        }
    }

    fn next<T: std::str::FromStr>(&mut self) -> T {
        self.tokens
            .next()
            .expect("input is missing a value")
            .parse()
            .ok()
            .expect("input contains an invalid value")
    }
}

fn solve(input: &str) -> String {
    let _scanner = Scanner::new(input);

    todo!("Implement the solution")
}

fn pour_kvas(barrels: &[i64], glass_volume: i64) -> i64 {
    let mut sum: i64 = barrels.iter().sum();
    let min_volume = *barrels.iter().min().unwrap();
    sum -= glass_volume;
    if sum < 0 {
        return -1;
    }
    std::cmp::min(min_volume, sum / (barrels.len() as i64))
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("{}", solve(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pour_kvas() {
        assert_eq!(pour_kvas(&[4, 3, 5], 3), 3);
        assert_eq!(pour_kvas(&[5, 3, 4], 4), 2);
        assert_eq!(pour_kvas(&[1, 2, 3], 7), -1);
    }
}

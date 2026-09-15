// 1100A: Roman and Browser
// Problem: https://codeforces.com/problemset/problem/1100/A
// Rating: 1000
// Tags: implementation
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

fn find_biggest_minus_mod_sum(tabs: &[i32], k: i32) -> i32 {
    let mut sums = vec![0; k as usize];
    let mut global_sum = 0;
    for i in 0..tabs.len() {
        sums[i % (k as usize)] += tabs[i];
        global_sum += tabs[i];
    }
    for sum in sums.iter_mut() {
        *sum = (global_sum - *sum).abs();
    }
    return *sums.iter().max().unwrap_or(&0);
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
    fn test_find_biggest_minus_mod_sum() {
        assert_eq!(find_biggest_minus_mod_sum(&[1, 1, -1, 1], 2), 2);
        assert_eq!(
            find_biggest_minus_mod_sum(&[-1, 1, -1, -1, 1, -1, -1, 1, -1, -1, 1, -1, -1, 1], 3),
            9
        );
    }
}

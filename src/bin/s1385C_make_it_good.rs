// 1385C: Make It Good
// Problem: https://codeforces.com/problemset/problem/1385/C
// Rating: 1200
// Tags: greedy
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

fn find_min_prefix(arr: &[i32]) -> usize {
    let mut i = arr.len() - 1;
    while i > 0 && arr[i - 1] >= arr[i] {
        i -= 1;
    }
    while i > 0 && arr[i - 1] <= arr[i] {
        i -= 1;
    }
    i
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("{}", solve(&input));
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn test_find_min_prefix() {
        assert_eq!(super::find_min_prefix(&[1, 2, 3, 4]), 0);
        assert_eq!(super::find_min_prefix(&[4, 3, 3, 8, 4, 5, 2]), 4);
        assert_eq!(super::find_min_prefix(&[1, 1, 1]), 0);
        assert_eq!(super::find_min_prefix(&[1, 3, 1, 4, 5, 3, 2]), 2);
        assert_eq!(super::find_min_prefix(&[5, 4, 3, 2, 3]), 3);
    }
}

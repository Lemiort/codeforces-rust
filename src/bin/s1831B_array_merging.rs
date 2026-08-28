// 1831B: Array merging
// Problem: https://codeforces.com/problemset/problem/1831/B
// Rating: 1000
// Tags: constructive algorithms, greedy
//
#![allow(dead_code)]

use std::{
    collections::HashMap,
    io::{self, Read},
};

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
    let mut scanner = Scanner::new(input);
    // number of test cases
    let n_cases: usize = scanner.next();
    let mut result = String::new();
    for _ in 0..n_cases {
        let n: usize = scanner.next();
        let mut values_a = Vec::with_capacity(n);
        let mut values_b = Vec::with_capacity(n);
        for _ in 0..n {
            values_a.push(scanner.next::<i64>());
        }
        for _ in 0..n {
            values_b.push(scanner.next::<i64>());
        }
        let longest = longest_conseqent_merge(&values_a, &values_b);
        println!("longest: {}", longest);
        result.push_str(&format!("{}\n", longest));
    }

    return result;
}

fn longest_equal(values: &[i64]) -> HashMap<i64, i64> {
    let mut longest = HashMap::new();
    longest.insert(values[0], 1);
    let mut prev = values[0];
    let mut current_length = 1;
    for i in 1..values.len() {
        // sequence of equals, just increase counter
        if values[i] == prev {
            current_length += 1;
        } else {
            // sequence of equals ended, update longest and reset counter
            let prev_length = longest.get(&prev).unwrap_or(&0);
            longest.insert(prev, std::cmp::max(current_length, *prev_length));
            current_length = 1;
        }
        prev = values[i];
    }
    let prev_length = longest.get(&prev).unwrap_or(&0);
    longest.insert(prev, std::cmp::max(current_length, *prev_length));
    longest
}

fn longest_conseqent_merge(values_a: &[i64], values_b: &[i64]) -> i64 {
    let longest_a = longest_equal(values_a);
    let longest_b = longest_equal(values_b);
    let longest = longest_a
        .iter()
        .filter_map(|(k, v)| {
            if let Some(v_b) = longest_b.get(k) {
                Some(v + v_b)
            } else {
                None
            }
        })
        .max()
        .unwrap_or(1);
    longest
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("{}", solve(&input));
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use super::{longest_conseqent_merge, longest_equal, solve};

    #[test]
    fn test_longest_equal() {
        assert_eq!(
            longest_equal(&[1, 1, 2, 2, 2, 3]),
            [(1, 2), (2, 3), (3, 1)].iter().cloned().collect()
        );
        assert_eq!(
            longest_equal(&[1, 1, 2, 3, 2, 2, 2]),
            [(1, 2), (2, 3), (3, 1)].iter().cloned().collect()
        );
    }

    #[test]
    fn test_longest_conseqent_merge() {
        assert_eq!(
            longest_conseqent_merge(&[1, 1, 2, 2, 2, 3], &[3, 3, 2, 2, 2, 1]),
            6
        );

        assert_eq!(longest_conseqent_merge(&[2], &[2]), 2);

        assert_eq!(longest_conseqent_merge(&[1, 2, 3], &[4, 5, 6]), 1);
    }

    #[test]
    fn test_solve() {
        let input = "4\n1\n2\n2\n3\n1 2 3\n4 5 6\n2\n1 2\n2 1\n5\n1 2 2 2 2\n2 1 1 1 1\n";
        let expected_output = "2\n1\n2\n5\n";
        assert_eq!(solve(input), expected_output);
    }
}

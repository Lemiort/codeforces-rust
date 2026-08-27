// 808D: Array Division
// Problem: https://codeforces.com/problemset/problem/808/D
// Rating: 1900
// Tags: binary search, data structures, implementation
//
#![allow(dead_code)]

use std::collections::HashMap;
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
    let mut scanner = Scanner::new(input);
    let n: usize = scanner.next();
    let values: Vec<i64> = (0..n).map(|_| scanner.next()).collect();

    if can_divide_after_one_move(&values) {
        "YES".to_owned()
    } else {
        "NO".to_owned()
    }
}

fn can_divide_after_one_move(values: &[i64]) -> bool {
    // Both parts must be non-empty, so an array with fewer than two values
    // cannot be split.
    if values.len() < 2 {
        return false;
    }

    let total: i64 = values.iter().sum();
    let mut left_sum = 0;
    let mut right_sum = total;

    // At each split, these maps contain the values currently available on
    // the left and right sides. Their counts also handle duplicate values.
    let mut left = HashMap::new();
    let mut right = HashMap::new();

    // Initially every value is on the right side of the split.
    for &value in values {
        *right.entry(value).or_insert(0usize) += 1;
    }

    // Leave at least one value on the right so both parts stay non-empty.
    for &value in &values[..values.len() - 1] {
        left_sum += value;
        right_sum -= value;
        decrement_count(&mut right, value);
        *left.entry(value).or_insert(0usize) += 1;

        // The array already has an equal-sum split; moving an element is
        // optional in the problem statement.
        if left_sum == right_sum {
            return true;
        }

        if left_sum > right_sum {
            let difference = left_sum - right_sum;
            // Moving x from left to right changes the difference by 2*x.
            if difference % 2 == 0 && left.contains_key(&(difference / 2)) {
                return true;
            }
        } else {
            let difference = right_sum - left_sum;
            // Moving x from right to left changes the difference by 2*x.
            if difference % 2 == 0 && right.contains_key(&(difference / 2)) {
                return true;
            }
        }
    }

    false
}

fn decrement_count(counts: &mut HashMap<i64, usize>, value: i64) {
    if let Some(count) = counts.get_mut(&value) {
        *count -= 1;
        if *count == 0 {
            counts.remove(&value);
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("{}", solve(&input));
}

#[cfg(test)]
mod tests {
    use super::{can_divide_after_one_move, solve};

    #[test]
    fn matches_problem_examples() {
        assert_eq!(solve("3\n1 3 2\n"), "YES");
        assert_eq!(solve("5\n1 2 3 4 5\n"), "NO");
        assert_eq!(solve("5\n2 2 3 4 5\n"), "YES");
    }

    #[test]
    fn handles_existing_split_and_small_inputs() {
        assert!(can_divide_after_one_move(&[1, 1]));
        assert!(can_divide_after_one_move(&[1, 2, 1]));
        assert!(!can_divide_after_one_move(&[7]));
    }

    #[test]
    fn handles_duplicate_values() {
        assert!(!can_divide_after_one_move(&[2, 2, 2, 2, 2]));
        assert!(!can_divide_after_one_move(&[1, 1, 1]));
    }
}

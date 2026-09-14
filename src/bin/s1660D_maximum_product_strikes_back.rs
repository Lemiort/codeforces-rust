// 1660D: Maximum Product Strikes Back
// Problem: https://codeforces.com/problemset/problem/1660/D
// Rating: 1600
// Tags: brute force, implementation, math, two pointers
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

fn find_maximum_product(nums: &[i32]) -> (usize, usize) {
    let n = nums.len();
    let mut max_twos = 0;
    // Initially, the answer is to remove the entire array (leave an empty segment with product 1)
    let mut best_k = n; // how many to remove from the left
    let mut best_r = 0; // how many to remove from the right

    let mut l = 0;
    while l < n {
        // Skip zeros
        if nums[l] == 0 {
            l += 1;
            continue;
        }

        // Find the right boundary of the current segment without zeros
        let mut r = l;
        while r < n && nums[r] != 0 {
            r += 1;
        }
        r -= 1; // Now the segment is nums[l..=r]

        // 1. Calculate characteristics of the segment
        let mut total_twos = 0;
        let mut neg_count = 0;
        let mut first_neg = None;
        let mut last_neg = None;

        for i in l..=r {
            if nums[i].abs() == 2 {
                total_twos += 1;
            }
            if nums[i] < 0 {
                neg_count += 1;
                if first_neg.is_none() {
                    first_neg = Some(i);
                }
                last_neg = Some(i);
            }
        }

        // 2. Calculate the best option for the current segment
        if neg_count % 2 == 0 {
            // Even number of negatives - take the entire segment
            if total_twos > max_twos {
                max_twos = total_twos;
                best_k = l;
                best_r = n - 1 - r;
            }
        } else {
            // Odd number of negatives - check two sub-options
            let fn_idx = first_neg.unwrap();
            let ln_idx = last_neg.unwrap();

            // Option A: cut off the left part up to the first negative inclusive
            let twos_a: i32 = (fn_idx + 1..=r)
                .map(|i| if nums[i].abs() == 2 { 1 } else { 0 })
                .sum();

            if twos_a > max_twos {
                max_twos = twos_a;
                best_k = fn_idx + 1;
                best_r = n - 1 - r;
            }

            // Option B: cut off the right part from the last negative inclusive
            let twos_b: i32 = (l..ln_idx)
                .map(|i| if nums[i].abs() == 2 { 1 } else { 0 })
                .sum();

            if twos_b > max_twos {
                max_twos = twos_b;
                best_k = l;
                best_r = n - ln_idx;
            }
        }

        l = r + 1; // Move to the next segment
    }

    (best_k, best_r)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("{}", solve(&input));
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use super::find_maximum_product;
    use super::solve;

    #[test]
    fn test_find_maximum_product() {
        assert_eq!(find_maximum_product(&[1, 2, -1, 2]), (0, 2));
        assert_eq!(find_maximum_product(&[1, 1, -2]), (3, 0));
        assert_eq!(find_maximum_product(&[2, 0, -2, 2, -1]), (0, 2));
        assert_eq!(find_maximum_product(&[-2, -1, -1]), (0, 1));
        assert_eq!(find_maximum_product(&[-1, -2, -2]), (1, 0));
    }

    #[test]
    #[ignore = "Replace with the problem's sample input and expected output"]
    fn sample_case() {
        assert_eq!(solve("sample input"), "sample output");
    }
}
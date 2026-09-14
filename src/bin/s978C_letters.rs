// 978C: Letters
// Problem: https://codeforces.com/problemset/problem/978/C
// Rating: 1000
// Tags: binary search, implementation, two pointers
//
#![allow(dead_code)]

use std::{
    io::{self, Read},
    result,
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
    let _scanner = Scanner::new(input);

    todo!("Implement the solution")
}

///return (building, flat number)
fn global_number_to_local(buildings: &[u64], mut letter: u64) -> (usize, u64) {
    for i in 0..buildings.len() {
        let flats_num = buildings[i];
        if letter <= flats_num {
            return (i + 1, letter);
        }
        letter -= flats_num;
    }
    (buildings.len(), letter)
}

fn global_numbers_to_locals(buildings: &[u64], letters: &[u64]) -> Vec<(usize, u64)> {
    let mut result = Vec::new();
    let mut current_sum = 0;
    let mut i = 0;

    for letter in letters {
        while current_sum + buildings[i] < *letter {
            current_sum += buildings[i];
            i += 1;
        }
        result.push((i + 1, *letter - current_sum));
    }

    result
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
    fn test_global_number_to_local() {
        assert_eq!(global_number_to_local(&[5, 6], 4), (1, 4));
        assert_eq!(global_number_to_local(&[5, 6], 6), (2, 1));
        assert_eq!(global_number_to_local(&[10, 15, 12], 37), (3, 12));
    }

    #[test]
    fn test_global_numbers_to_locals() {
        assert_eq!(
            global_numbers_to_locals(&[10, 15, 12], &[1, 9, 12, 23, 26, 37]),
            vec![(1, 1), (1, 9), (2, 2), (2, 13), (3, 1), (3, 12),]
        );

        assert_eq!(
            global_numbers_to_locals(&[5, 10000000000], &[5, 6, 9999999999]),
            vec![(1, 5), (2, 1), (2, 9999999994),]
        );
    }
}

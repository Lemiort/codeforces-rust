// 1020B: Badge
// Problem: https://codeforces.com/problemset/problem/1020/B
// Rating: 1000
// Tags: brute force, dfs and similar, graphs
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
    let mut scanner = Scanner::new(input);

    let n = scanner.next();
    let edges = (0..n).map(|_| scanner.next::<usize>()).collect::<Vec<_>>();

    let badges = calculate_badge_distribution(n, &edges);
    badges
        .into_iter()
        .map(|b| b.to_string())
        .collect::<Vec<_>>()
        .join(" ")
}

fn calculate_badge_distribution(n: usize, edges: &[usize]) -> Vec<usize> {
    let mut badges = vec![-1i32; n];

    #[derive(Debug, Clone, PartialEq, Eq)]
    enum State {
        Unvisited,
        Visiting,
        Visited,
    }
    let mut states = vec![State::Unvisited; n];

    for i_start in 0..n {
        if badges[i_start] != -1 {
            continue; // already assigned a badge
        }

        let mut i = i_start;
        // go over all nodes
        while states[i] == State::Unvisited {
            states[i] = State::Visiting;
            i = edges[i] - 1;
        }

        // if we reach a node that is already visited, it means we have found a cycle
        if states[i] == State::Visiting {
            let cycle_start = i;
            // go from start to cycle start and assign badges
            i = i_start;
            while i != cycle_start {
                badges[i] = (cycle_start + 1) as i32;
                i = edges[i] - 1;
                states[i] = State::Visited;
            }

            // go over the cycle from cycle start and assign badges
            i = cycle_start;
            while badges[i] == -1 {
                badges[i] = (i + 1) as i32;
                i = edges[i] - 1;
                states[i] = State::Visited;
            }
        } else {
            // if we reach a node that is already visited, it means we have reached a node that has already been assigned a badge
            let badge = badges[i];
            i = i_start;
            while badges[i] == -1 {
                badges[i] = badge;
                i = edges[i] - 1;
                states[i] = State::Visited;
            }
        }
    }

    return badges.into_iter().map(|b| b as usize).collect();
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
    fn test_calculate_badge_distribution() {
        let n = 3;
        let edges = vec![2, 3, 2];
        let expected_badges = vec![2, 2, 3]; // Example expected output
        assert_eq!(
            super::calculate_badge_distribution(n, &edges),
            expected_badges
        );

        let n = 3;
        let edges = vec![1, 2, 3];
        let expected_badges = vec![1, 2, 3]; // Example expected output
        assert_eq!(
            super::calculate_badge_distribution(n, &edges),
            expected_badges
        );
    }

    #[test]
    #[ignore = "Replace with the problem's sample input and expected output"]
    fn sample_case() {
        assert_eq!(solve("sample input"), "sample output");
    }
}

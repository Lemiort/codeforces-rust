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
    let mut in_degree = vec![0; n];
    let mut ans = vec![0; n];

    for &to in edges {
        in_degree[to - 1] += 1;
    }

    // find all vectrices not in cycles
    let mut queue = Vec::new();
    for i in 0..n {
        if in_degree[i] == 0 {
            queue.push(i);
        }
    }

    // cut all vertices not in cycles
    let mut head = 0;
    while head < queue.len() {
        let u = queue[head];
        head += 1;
        let v = edges[u] - 1;
        in_degree[v] -= 1;
        if in_degree[v] == 0 {
            queue.push(v);
        }
    }

    // all renaming vertices are in cycles, we can assign them badges
    for i in 0..n {
        if in_degree[i] > 0 {
            ans[i] = i + 1;
        }
    }

    // for each vertex in the queue, assign it the same badge as its parent
    for &u in queue.iter().rev() {
        let v = edges[u] - 1;
        ans[u] = ans[v];
    }

    ans
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

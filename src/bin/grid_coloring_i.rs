use std::collections::HashSet;
use std::io::{Read, stdin};

fn main() {
    let mut input: String = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let head = lines.next().unwrap();
    let grid: Vec<&str> = lines.collect();
    let mut head_iter = head
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap());
    let n = head_iter.next().unwrap() as usize;
    let m = head_iter.next().unwrap() as usize;
    let grid: Vec<Vec<char>> = grid
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();
    let mut ans: Vec<Vec<char>> = vec![vec!['E'; m]; n];
    for i in 0..n {
        for j in 0..m {
            let mut choice = HashSet::from(['A', 'B', 'C', 'D']);
            choice.remove(&grid[i][j]);
            if i > 0 {
                choice.remove(&ans[i - 1][j]);
            }
            if j > 0 {
                choice.remove(&ans[i][j - 1]);
            }
            ans[i][j] = choice.into_iter().next().unwrap();
        }
    }
    println!(
        "{}",
        ans.into_iter()
            .map(|line| line
                .into_iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
                .join(""))
            .collect::<Vec<String>>()
            .join("\n")
    )
}

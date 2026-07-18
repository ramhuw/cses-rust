use std::io::{self, Read};

const P: usize = 1000000007;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let mut nm = lines.next().unwrap().split_whitespace();
    let n = nm.next().unwrap().parse::<usize>().unwrap();
    let m = nm.next().unwrap().parse::<usize>().unwrap();
    let xs: Vec<usize> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|a| a.parse::<usize>().unwrap())
        .collect();
    let mut store: Vec<Vec<usize>> = vec![vec![0; m + 1]; n];
    for i in 0..n {
        if i == 0 {
            if xs[i] == 0 {
                for j in 1..=m {
                    store[i][j] = 1;
                }
            } else {
                store[i][xs[i]] = 1;
            }
        } else {
            if xs[i] == 0 {
                for j in 1..=m {
                    store[i][j] += store[i - 1][j];
                    if j > 0 {
                        store[i][j] += store[i - 1][j - 1];
                    }
                    if j < m {
                        store[i][j] += store[i - 1][j + 1];
                    }
                    store[i][j] %= P;
                }
            } else {
                store[i][xs[i]] += store[i - 1][xs[i]];
                if xs[i] > 0 {
                    store[i][xs[i]] += store[i - 1][xs[i] - 1];
                }
                if xs[i] < m {
                    store[i][xs[i]] += store[i - 1][xs[i] + 1];
                }
                store[i][xs[i]] %= P;
            }
        }
    }
    println!("{}", store[n-1].iter().fold(0, |acc, a| (acc + a) % P));
}

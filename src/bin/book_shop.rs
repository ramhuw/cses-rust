use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let mut nx = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|a| a.parse::<usize>().unwrap());
    let n = nx.next().unwrap();
    let x = nx.next().unwrap();

    let mut hs = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|a| a.parse::<usize>().unwrap());

    let mut ss = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|a| a.parse::<usize>().unwrap());

    let mut store: Vec<usize> = vec![0; x + 1];
    for _ in 0..n {
        let hi = hs.next().unwrap();
        let si = ss.next().unwrap();
        if x >= hi {
            for j in (0..=x.saturating_sub(hi)).rev() {
            store[j + hi] = store[j + hi].max(store[j] + si);
        }
        }
        
    }
    println!("{}", store[x]);
}

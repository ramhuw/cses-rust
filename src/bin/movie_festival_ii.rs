use std::{collections::BTreeSet, io::Read, ops::Bound::{Included, Unbounded}};


fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();
    let mut set: BTreeSet<(usize, usize, usize)> = BTreeSet::new();
    let n: usize = tokens.next().unwrap().parse().unwrap();
    let k: usize = tokens.next().unwrap().parse().unwrap();
    let mut interval: Vec<(usize, usize)> = vec![];
    for i in 0..k {
        set.insert((0, 0, i));
    }
    for _ in 0..n {
        let a: usize = tokens.next().unwrap().parse().unwrap();
        let b: usize = tokens.next().unwrap().parse().unwrap();
        interval.push((a, b))
    }
    interval.sort_unstable_by_key(|&(_, b)| b);
    for i in 0..n {
        if let Some(&(end_time, count, id)) = set.range((Unbounded, Included((interval[i].0, usize::MAX, usize::MAX)))).next_back() {
            set.remove(&(end_time, count, id));
            set.insert((interval[i].1, count + 1, id));
        }
    }
    println!("{}", set.iter().fold(0, |acc, (_, c, _)| acc + c));
}
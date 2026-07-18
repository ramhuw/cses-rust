use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let mut nq = lines.next().unwrap().split_whitespace().map(|a| a.parse::<usize>().unwrap());
    let _ = nq.next().unwrap();
    let _ = nq.next().unwrap();
    let xs: Vec<u64> = lines.next().unwrap().split_whitespace().map(|a| a.parse::<u64>().unwrap()).collect();
    let mut s = 0;
    let mut ss: Vec<u64> = vec![0];
    for x in xs {
        s += x;
        ss.push(s);
    }
    let mut ans: String = String::new();
    while let Some(ab) = lines.next() {
        let ab_vec: Vec<usize> = ab.split_whitespace().map(|a| a.parse::<usize>().unwrap()).collect();
        let a = ab_vec[0];
        let b = ab_vec[1];
        let result = ss[b] - ss[a-1];
        ans += &(result.to_string() + "\n")
    }
    println!("{}", ans);
}
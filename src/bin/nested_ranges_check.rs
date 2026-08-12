use std::{fmt::Write, io::Read, cmp::Reverse};


fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();
    let n: usize = tokens.next().unwrap().parse().unwrap();
    let mut ranges: Vec<(u32, u32, usize)> = vec![];
    for i in 0..n {
        let x: u32 = tokens.next().unwrap().parse().unwrap();
        let y: u32 = tokens.next().unwrap().parse().unwrap();
        ranges.push((x, y, i));
    }
    ranges.sort_by_key(|&(a, b, c)| (a, Reverse(b), c));
    let mut m = u32::MAX;
    let mut ms: Vec<u32> = vec![0; n];
    for i in (0..n).rev() {
        m = m.min(ranges[i].1);
        ms[i] = m;
    }
    m = 0;
    let mut mas: Vec<u32> = vec![];
    for i in 0..n {
        m = m.max(ranges[i].1);
        mas.push(m);
    }
    let mut ans1: Vec<bool> = vec![false; n];
    let mut ans2: Vec<bool> = vec![false; n];
    for i in 0..n {
        if i < n - 1 && ms[i+1] <= ranges[i].1 {
            ans1[ranges[i].2] = true;
        }
        if i > 0 && mas[i-1] >= ranges[i].1 {
            ans2[ranges[i].2] = true;
        }
    }
    let mut ans = String::new();
    for b in ans1 {
        let a = if b {
            1
        } else {
            0
        };
        write!(ans, "{} ", a).unwrap();
    }
    ans.push('\n');
    for b in ans2 {
        let a = if b {
            1
        } else {
            0
        };
        write!(ans, "{} ", a).unwrap();
    }
    println!("{ans}");
}
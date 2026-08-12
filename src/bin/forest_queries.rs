use std::{fmt::Write, io::Read};

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();
    let n: usize = tokens.next().unwrap().parse().unwrap();
    let q: usize = tokens.next().unwrap().parse().unwrap();
    let mut d: Vec<u32> = vec![0; (n + 1) * (n + 1)];
    for i in 0..n {
        let mut line = tokens.next().unwrap().chars();
        for j in 0..n {
            let c = line.next().unwrap();
            d[(i + 1) * (n + 1) + j + 1] = d[i * (n + 1) + j + 1] + d[(i + 1) * (n + 1) + j] - d[i * (n + 1) + j];
            if c == '*' {
                d[(i + 1) * (n + 1) + j + 1] += 1;
            }
        }
    }
    let mut ans = String::new();
    for _ in 0..q {
        let y1: usize = tokens.next().unwrap().parse().unwrap();
        let x1: usize = tokens.next().unwrap().parse().unwrap();
        let y2: usize = tokens.next().unwrap().parse().unwrap();
        let x2: usize = tokens.next().unwrap().parse().unwrap();
        write!(
            ans,
            "{}\n",
            d[y2 * (n + 1) + x2] - d[y2 * (n + 1) + x1 - 1] - d[(y1 - 1) * (n + 1) + x2] + d[(y1 - 1) * (n + 1) + x1 - 1]
        )
        .unwrap();
    }
    println!("{ans}");
}

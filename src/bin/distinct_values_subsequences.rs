use std::{collections::HashMap, io::Read};
const P: u64 = 1000000007;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();
    let n: usize = tokens.next().unwrap().parse().unwrap();
    let mut count: HashMap<u64, u64> = HashMap::new();
    let mut ans: u64 = 1;
    for _ in 0..n {
        let x: u64 = tokens.next().unwrap().parse().unwrap();
        let a = count.entry(x).or_insert(0);
        ans += ans * inv(*a + 1) % P;
        ans %= P;
        *a += 1;
    }
    println!("{}", (ans + P - 1) % P);
}

fn exp(a: u64, b: u64) -> u64 {
    let mut ans = 1;
    let mut x = a;
    let mut y = b % (P - 1);
    while y != 0 {
        if y & 1 == 1 {
            ans = (ans * x) % P;
        }
        x = (x * x) % P;
        y >>= 1;
    }
    ans
}

fn inv(a: u64) -> u64 {
    exp(a, P - 2)
}
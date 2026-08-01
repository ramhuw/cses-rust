use std::collections::HashMap;
const P: usize = 1000000007;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut f: HashMap<char, usize> = HashMap::new();
    let s = input.trim();
    let n = s.len();
    let mut fact: Vec<usize> = vec![1];
    let mut k = 1;
    for i in 1..=n {
        k = (k * i) % P;
        fact.push(k);
    }
    for c in s.chars() {
        *f.entry(c).or_insert(0) += 1;
    }
    let mut ans = fact[n];
    for (_, v) in f {
        ans = ans * inv(fact[v]) % P;
    }
    println!("{ans}");
}

fn exp(a: usize, b: usize) -> usize {
    let mut ans = 1;
    let mut x = a % P;
    let mut y = b % (P - 1);
    while y != 0 {
        if y & 1 == 1 {
            ans = ans * x % P;
        }
        x = x * x % P;
        y >>= 1;
    }
    ans
}

fn inv(a: usize) -> usize {
    exp(a, P - 2)
}
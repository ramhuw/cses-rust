use std::{collections::{HashMap, HashSet}, io::Read};
const P: usize = 1000000007;
fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace().map(|x| x.parse::<usize>().unwrap());
    let n = tokens.next().unwrap();
    let mut primes_bucket: Vec<bool> = vec![true; n+1];
    primes_bucket[0] = false;
    primes_bucket[1] = false;
    let mut primes: Vec<usize> = vec![];
    for p in 2..=n {
        if primes_bucket[p] {
            primes.push(p);
            for q in (2*p..=n).step_by(p) {
                primes_bucket[q] = false;
            }
        }
    }
    
    let mut prime_factors: HashMap<usize, usize> = HashMap::new();
    
    let ps: Vec<usize> = tokens.collect();
    let mut searched: HashSet<usize> = HashSet::new();
    for i in 1..n {
        if searched.contains(&i) {
            continue;
        }
        searched.insert(i);
        let mut count: usize = 1;
        let mut current = ps[i - 1];
        while current != i {
            count += 1;
            searched.insert(current);
            current = ps[current - 1];
        }
        let mut i = 0;
        let mut p = primes[i];
        while count != 1 {
            while count % p != 0 {
                i += 1;
                p = primes[i];
                if p * p > count {
                    p = count;
                }
            }
            let mut k = 0;
            while count % p == 0 {
                k += 1;
                count /= p;
            }
            let a = prime_factors.entry(p).or_insert(0);
            *a = k.max(*a);
            p += 1;
        }
    }
    let mut ans: usize = 1;
    for (p, k) in prime_factors {
        ans = (ans * exp(p, k)) % P;
    }
    println!("{}", ans);
}


fn exp(a: usize, b: usize) -> usize {
    let mut ans = 1;
    let mut x = a;
    let mut y = b;
    while y != 0 {
        if y & 1 == 1 {
            ans = (ans * x) % P;
        }
        x = (x * x) % P;
        y >>= 1;
    }
    ans
}
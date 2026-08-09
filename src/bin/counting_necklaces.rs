use std::io::Read;
const P: usize = 1000000007;

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

fn inv(a: usize) -> usize {
    exp(a, P - 2)
}
fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap());
    let n = tokens.next().unwrap();
    let m = tokens.next().unwrap();
    let mut primes: Vec<usize> = vec![];
    let mut powers: Vec<usize> = vec![];
    let mut p: usize = 2;
    let mut worker: usize = n;
    while worker != 1 {
        while worker % p != 0 {
            p += 1;
            if p * p > worker {
                p = worker;
            }
        }
        let mut k: usize = 0;
        while worker % p == 0 {
            k += 1;
            worker /= p;
        }
        primes.push(p);
        powers.push(k);
        p += 1;
    }
    let mut ans: usize = 0;
    let mut worker: Vec<usize> = vec![0; primes.len()];
    deal(&primes, &powers, &mut worker, 0, &mut ans, n, m);
    ans = ans * inv(n) % P;
    println!("{ans}");
}

fn deal(
    primes: &Vec<usize>,
    powers: &Vec<usize>,
    worker: &mut Vec<usize>,
    pos: usize,
    ans: &mut usize,
    n: usize,
    m: usize,
) {
    if pos >= worker.len() {
        let mut a: usize = 1;
        let mut d: usize = 1;
        for i in 0..worker.len() {
            if worker[i] > 0 {
                let c = exp(primes[i], worker[i] - 1);
                a *= c * (primes[i] - 1);
                d *= c * primes[i];
            }
        }
        let b = exp(m, n / d);
        *ans = (*ans + a * b % P) % P;
    } else {
        for j in 0..=powers[pos] {
            worker[pos] = j;
            deal(&primes, &powers, worker, pos + 1, ans, n, m);
        }
    }
}

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
    exp(a, P-2)
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<usize>().unwrap();
    if n & 1 == 1 {
        println!("0");
        return;
    }
    let m = n / 2;
    input.clear();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut l: usize = 0;
    let mut r: usize = 0;
    for c in input.trim().chars() {
        if c == '(' {
            l += 1;
        } else {
            r += 1;
        }
        if r > l || l > m || r > m {
            println!("0");
            return;
        }
    }
    let k = l + r;
    if k == n || m == l {
        println!("1");
        return;
    }
    let mut fact: Vec<usize> = vec![1];
    for i in 1..=n {
        fact.push(fact[i-1] * i % P);
    }
    let ans = fact[n-k] * (inv(fact[m-l] * fact[n+l-k-m] % P) + P - inv(fact[m+1-r] * fact[n+r-k-m-1] % P)) % P;
    println!("{ans}");
}
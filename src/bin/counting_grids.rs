const P: usize = 1000000007;
fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();
    let h: usize = if n & 1 == 1 {(n * n - 1) / 2 + 1} else {n * n / 2};
    let q = if n & 1 == 1 {(n * n - 1) / 4 + 1} else {n * n / 4};

    let mut ans: usize = exp(2, n * n);
    ans += exp(2, h);
    ans %= P;
    ans += (2 * exp(2, q)) % P;
    ans %= P;
    ans *= inv(4);
    ans %= P;
    println!("{ans}");
}

fn exp(a: usize, b: usize) -> usize {
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

fn inv(a: usize) -> usize {
    exp(a, P - 2)
}
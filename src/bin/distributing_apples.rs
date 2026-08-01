const P: usize = 1000000007;
fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut nm = input.split_whitespace().map(|a| a.parse::<usize>().unwrap());
    let n = nm.next().unwrap();
    let m = nm.next().unwrap();
    let mut a = 1;
    let mut b = 1;
    let mut c = 1;
    let mut f = 1;
    for i in 1..=(m+n-1) {
        f = (f * i) % P;
        if i == m + n - 1 {
            a = f;
        }
        if i == n - 1 {
            b = f;
        }
        if i == m {
            c = f;
        }
    }
    println!("{}", a * inv(b) % P * inv(c) % P);
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
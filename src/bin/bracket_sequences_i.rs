const P: usize = 1000000007;
fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<usize>().unwrap();
    if n & 1 == 1 {
        println!("0");
        return;
    }
    let m = n / 2;
    let mut fact: usize = 1;
    for i in 1..=m {
        fact = fact * i % P;
    }
    let f1 = fact;
    fact = fact * (m+1) % P;
    let f2 = fact;
    for i in (m+2)..=n {
        fact = fact * i % P;
    }
    let f3 = fact;
    let ans = f3 * inv(f2 * f1 % P) % P;
    println!("{ans}");
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

fn inv(a: usize) -> usize {
    exp(a, P-2)
}
const P: usize = 1000000007;
fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<usize>().unwrap();
    let s = (n + 1) * n / 2;
    if s & 1 == 1 {
        println!("0");
        return;
    }
    let mut d = vec![0; s/2+1];
    d[0] = 1;
    for i in 1..=n {
        for j in (0..=(s/2-i)).rev() {
            d[j+i] = (d[j+i] + d[j]) % P;
        }
    }
    println!("{}", d[s/2] * inv(2) % P);
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
    exp(a, P - 2)
}
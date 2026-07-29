use std::io::Read;
const P: u128 = 1000000007;
fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let n = lines.next().unwrap().parse::<usize>().unwrap();
    let mut number: u128 = 1;
    let mut number2: u128 = 1;
    let mut sum: u128 = 1;
    let mut product: u128 = 1;
    for _ in 0..n {
        let mut line = lines.next().unwrap().split_whitespace().map(|x| x.parse::<u128>().unwrap());
        let x = line.next().unwrap();
        let k = line.next().unwrap();
        product = exp(product, k+1) * exp(x, (1 + k)*k/2 * number2) % P;
        number = number * (k + 1) % P;
        number2 = number2 * (k + 1) % (P - 1);
        sum = sum * (exp(x, k + 1) - 1) % P;
        let mut rec = bezout(x as i128 - 1, P as i128).0;
        while rec < 0 {
            rec += P as i128;
        }
        sum = sum * rec as u128 % P;
    }
    println!("{} {} {}", number, sum, product);
}

fn exp(a: u128, b: u128) -> u128 {
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

fn bezout(a: i128, b: i128) -> (i128, i128) {
    if a == 1 {
        return (1, 0);
    }
    if a == -1 {
        return (-1, 0);
    }
    if b == 1 {
        return (0, 1);
    }
    if b == -1 {
        return (0, -1);
    }
    let (x, y) = if a.abs() >= b.abs() {(a, b)} else {(b, a)};
    let r = x % y;
    let q = x / y;
    let (u, vqu) = bezout(r, y);
    let v = vqu - q * u;
    return if x == a {(u, v)} else {(v, u)}
}
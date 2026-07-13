const P: u128 = 1000000007;

fn count(x: u128, p: u128) -> u128 {
    let mut ans: u128 = 0;
    let mut i = 1;
    while i * i <= x {
        ans += (i * (x / i - i + 1)) % p;
        ans %= p;
        if i + 1 <= x / i {
            ans += ((i + 1 + (x / i)) * (x / i - i)) / 2 % p;
            ans %= p;
        }

        i += 1;
    }
    ans
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let n: u128 = input.trim().parse().unwrap();
    println!("{}", count(n, P));
}

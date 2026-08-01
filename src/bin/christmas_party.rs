const P: usize = 1000000007;
fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<usize>().unwrap();
    let mut fact: Vec<usize> = vec![1];
    
    for i in 1..=n {
        fact.push(fact.last().unwrap() * i % P);
    }
    let mut ans = 0;
    let mut flag = true;
    for i in 0..=n {
        let temp = fact[n - i] * fact[n] % P * inv(fact[i]) % P * inv(fact[n-i]) % P;
        if flag {
            ans += temp;
        } else {
            ans += P - temp;
        }
        ans %= P;
        flag = !flag;
    }
    println!("{}", ans);
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
use std::io::Read;
const P: usize = 1000000007;
fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines().map(|line| line.split_whitespace().map(|x| x.parse::<usize>().unwrap()));
    let _ = lines.next().unwrap().next().unwrap();
    let mut ans = String::new();
    let mut fact: Vec<usize> = vec![1];
    let mut k = 1;
    for i in 1..=1000000 {
        k = (k * i) % P;
        fact.push(k);
    }
    for mut line in lines {
        let a = line.next().unwrap();
        let b = line.next().unwrap();
        let bi = fact[a] * inv(fact[b]) % P * inv(fact[a-b]) % P;
        ans.push_str(&bi.to_string());
        ans.push('\n');
    }
    print!("{ans}");
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
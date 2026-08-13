use std::io::Read;
fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();
    let n: i64 = tokens.next().unwrap().parse().unwrap();
    let mut f = vec![0usize; n as usize];
    let mut s: i64 = 0;
    f[0] = 1;
    let mut ans: usize = 0;
    for a in tokens.map(|a| a.parse::<i64>().unwrap()) {
        s += a;
        let m = s.rem_euclid(n) as usize;
        ans += f[m];
        f[m] += 1;
    }
    println!("{ans}");
}

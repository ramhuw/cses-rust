use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap());
    let n = tokens.next().unwrap();
    let t = tokens.next().unwrap();
    let mut ks: Vec<usize> = vec![];
    let mut ma: usize = 0;
    let mut mi = usize::MAX;
    for _ in 0..n {
        let k = tokens.next().unwrap();
        ks.push(k);
        ma = ma.max(k);
        mi = mi.min(k);
    }
    let mut left = t * mi / n;
    let mut right = t * ma;
    while left < right {
        let middle = (left + right) / 2;
        if ks.iter().fold(0, |acc, a| acc + middle / a) >= t {
            right = middle;
        } else {
            left = middle + 1;
        }
    }
    println!("{}", right);
}

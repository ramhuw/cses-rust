use std::io;

const P: u128 = 1000000007;

fn main() {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let q: u128 = input.trim().parse().unwrap();
    for _ in 0..q {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let mut iter = line
            .trim()
            .split_whitespace()
            .map(|x| x.to_string().parse::<u128>().unwrap());
        let a = iter.next().unwrap();
        let b = iter.next().unwrap();
        println!("{}", exp(a, b));
    }
}

fn exp(a: u128, b: u128) -> u128 {
    let mut aa: Vec<u128> = vec![a];
    let mut bb = vec![];
    let mut b = b;
    while b != 0 {
        bb.push(b & 1);
        b >>= 1;
        let al = aa.last().unwrap();
        aa.push((al * al) % P);
    }
    let mut result = 1;
    for i in 0..(bb.len()) {
        if bb[i] == 1 {
            result = (result * aa[i]) % P;
        }
    }
    result
}

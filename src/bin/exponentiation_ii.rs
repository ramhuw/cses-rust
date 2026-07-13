use std::io::Read;

const P: u64 = 1000000007;

fn exp(a: u64, b: u64, p: u64) -> u64 {
    let mut aa: Vec<u64> = vec![a];
    let mut bb = vec![];
    let mut b = b;
    while b != 0 {
        bb.push(b & 1);
        b >>= 1;
        let al = aa.last().unwrap();
        aa.push((al * al) % p);
    }
    let mut result = 1;
    for i in 0..(bb.len()) {
        if bb[i] == 1 {
            result = (result * aa[i]) % p;
        }
    }
    result
}

fn expo(a: u64, b: u64, c: u64, p: u64) -> u64 {
    exp(a, exp(b, c, p - 1), p)
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let _ = lines.next().unwrap();
    while let Some(line) = lines.next() {
        let mut abc = line.split_whitespace().map(|x| x.parse::<u64>().unwrap());
        let a = abc.next().unwrap();
        let b = abc.next().unwrap();
        let c = abc.next().unwrap();
        println!("{}", expo(a, b, c, P));
    }
}

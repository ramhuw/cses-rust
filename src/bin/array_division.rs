use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();
    let _ = tokens.next().unwrap();
    let k: usize = tokens.next().unwrap().parse().unwrap();
    let x: Vec<usize> = tokens.map(|a| a.parse().unwrap()).collect();

    let mut max = 0;
    let mut sum = 0;
    for &a in &x {
        max = a.max(max);
        sum += a;
    }
    let s = if sum % k == 0 { sum / k } else { sum / k + 1 };
    let mut l = max.max(s);
    let mut u = sum;
    while l < u {
        let m = (l + u) / 2;
        if search(&x, m, k) {
            u = m
        } else {
            l = m + 1;
        }
    }
    println!("{l}");
}

fn search(x: &[usize], m: usize, k: usize) -> bool {
    let mut g = 0;
    let mut c = 0;
    for i in 0..x.len() {
        if c + x[i] > m {
            c = x[i];
            g += 1;
        } else {
            c += x[i];
        }
        if i == x.len() - 1 {
            g += 1;
        }
        if g > k {
            return false;
        }
    }
    true
}

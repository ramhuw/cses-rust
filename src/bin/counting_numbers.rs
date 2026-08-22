use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();
    let a: usize = tokens.next().unwrap().parse().unwrap();
    let b: usize = tokens.next().unwrap().parse().unwrap();
    let mut ans = f(b);
    if a != 0 {
        ans -= f(a - 1);
    }
    println!("{}", ans);
}

fn f(a: usize) -> usize {
    if a == 0 {
        return 1;
    }
    let mut e = vec![];
    let mut b = a;
    while b != 0 {
        e.push(b % 10);
        b /= 10;
    }
    e.reverse();
    let mut flags = vec![true];
    let mut flag = true;
    for i in 1..e.len() {
        if e[i] == e[i - 1] {
            flag = false;
        }
        flags.push(flag);
    }
    let mut d = vec![0; 10 * e.len()];
    for i in 0..e.len() {
        for j in 0..=9 {
            if i == 0 {
                if j != 0 && j <= e[0] {
                    d[i * 10 + j] = 1;
                }
            } else {
                for k in 0..=9 {
                    if k != j {
                        d[i * 10 + j] += d[(i - 1) * 10 + k];
                    }
                }
                if j != 0 {
                    d[i * 10 + j] += 1;
                }
                if j > e[i] && flags[i - 1] && j != e[i - 1] {
                    d[i * 10 + j] -= 1;
                }
            }
        }
    }
    let mut ans = 0;
    for i in 0..10 {
        ans += d[(e.len() - 1) * 10 + i];
    }
    ans + 1
}

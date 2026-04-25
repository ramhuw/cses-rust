use std::io;

fn main() {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let q: u32 = input.trim().parse().unwrap();
    for _ in 0..q {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let mut iter = line
            .trim()
            .split_whitespace()
            .map(|x| x.to_string().parse::<u32>().unwrap());
        let n = iter.next().unwrap();
        let k = iter.next().unwrap();
        println!("{}", j(n, k));
    }
}

fn j(n: u32, k: u32) -> u32 {
    if n == 1 {
        return 1;
    } else if k == 1 {
        return 2;
    }
    if 2 * k <= n {
        return 2 * k;
    } else {
        if n % 2 == 0 {
            2 * j(n / 2, k - n / 2) - 1
        } else {
            let s = j(n / 2 + 1, k - n / 2);
            if s == 1 {
                return n;
            }
            return 2 * s - 3;
        }
    }
}

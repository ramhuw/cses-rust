use std::io::Read;
use std::fmt::Write;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace().map(|x| x.parse::<usize>().unwrap());
    let t = tokens.next().unwrap();
    let mut ans = String::new();
    'outer: for _ in 0..t {
        let n = tokens.next().unwrap();
        let k = 8 * n + 1;
        let m = isqrt(k);
        if m * m == k {
            write!(ans, "1\n", ).unwrap();
            continue;
        }
        let mut l = k + 1;
        let mut p = 2;
        while l != 1 {
            while l % p != 0 {
                p += 1;
                if p * p > l {
                    p = l;
                }
            }
            if p % 4 == 3 {
                let mut q = 0;
                while l % p == 0 {
                    l /= p;
                    q += 1;
                }
                if q & 1 == 1 {
                    write!(ans, "3\n").unwrap();
                    continue 'outer;
                }
            } else {
                while l % p == 0 {
                    l /= p;
                }
            }
            p += 1;
        }
        write!(ans, "2\n").unwrap();
    }
    println!("{ans}");
}

fn isqrt(n: usize) -> usize {
    let mut left = 0;
    let mut right = n;
    while left < right {
        let middle = (left + right + 1) / 2;
        let middle_square = middle.saturating_mul(middle);
        if middle_square <= n {
            left = middle;
        } else {
            right = middle - 1;
        }
    }
    left
}
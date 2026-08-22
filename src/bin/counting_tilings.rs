use std::io::Read;
const P: u64 = 1000000007;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();
    let n: usize = tokens.next().unwrap().parse().unwrap();
    let m: usize = tokens.next().unwrap().parse().unwrap();
    let mut d: Vec<u64> = vec![0; (m + 1) * (1 << n)];
    d[c(0, 0, n)] = 1;
    for col in 0..m {
        for mask in 0..(1 << n) {
            let free = !mask & ((1 << n) - 1);
            let mut search = vec![(0, 0)];
            while let Some((new_mask, i)) = search.pop() {
                if i == n {
                    d[c(col + 1, new_mask, n)] += d[c(col, mask, n)];
                    d[c(col + 1, new_mask, n)] %= P;
                } else if free & (1 << i) != 0 {
                    search.push((new_mask | (1 << i), i + 1));
                    if i + 1 < n && free & (1 << (i + 1)) != 0 {
                        search.push((new_mask, i + 2));
                    }
                } else {
                    search.push((new_mask, i + 1));
                }
            }
        }
    }
    println!("{:?}", d[c(m, 0, n)]);
}

fn c(col: usize, mask: usize, n: usize) -> usize {
    col * (1 << n) + mask
}

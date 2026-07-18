use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let mut nq = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|a| a.parse::<usize>().unwrap());
    let n = nq.next().unwrap();
    let _ = nq.next().unwrap();
    let xs: Vec<u64> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|a| a.parse::<u64>().unwrap())
        .collect();
    let mut ms: Vec<Vec<u64>> = vec![xs];
    for i in 1.. {
        if 1 << i > n {
            break;
        }
        let mut m: Vec<u64> = Vec::new();
        for j in 0.. {
            if j + (1 << i) > n {
                break;
            }
            m.push(ms[i - 1][j].min(ms[i - 1][j + (1 << (i - 1))]))
        }
        ms.push(m);
    }

    let mut ans: String = String::new();

    while let Some(ab) = lines.next() {
        let ab_vec: Vec<usize> = ab
            .split_whitespace()
            .map(|a| a.parse::<usize>().unwrap())
            .collect();
        let mut left = ab_vec[0] - 1;
        let right = ab_vec[1];
        let mut l = right - left;
        let mut result = u64::MAX;
        for i in 0usize.. {
            if l == 0 {
                break;
            }
            if l & 1 == 1 {
                result = result.min(ms[i][left]);
                left += 1 << i;
            }
            l = l >> 1;
            
        }

        ans += &(result.to_string() + "\n")
    }
    println!("{}", ans);
}

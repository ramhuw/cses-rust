use std::io::Read;
#[derive(Clone, Copy)]
enum Distance {
    Finite(usize),
    Infinite,
}

impl Distance {
    fn add(&self, other: &Self) -> Self {
        match (self, other) {
            (Self::Finite(x), Self::Finite(y)) => Self::Finite(x + y),
            _ => Self::Infinite,
        }
    }

    fn min(&self, other: &Self) -> Self {
        match (self, other) {
            (Self::Finite(x), Self::Finite(y)) => {
                if x > y {
                    Self::Finite(*y)
                } else {
                    Self::Finite(*x)
                }
            }
            (left, Self::Infinite) => *left,
            (Self::Infinite, right) => *right,
        }
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap());
    let n = tokens.next().unwrap();
    let m = tokens.next().unwrap();
    let q = tokens.next().unwrap();
    let mut d: Vec<Vec<Distance>> = (0..n)
        .map(|i| {
            (0..n)
                .map(|j| {
                    if i == j {
                        Distance::Finite(0)
                    } else {
                        Distance::Infinite
                    }
                })
                .collect()
        })
        .collect();
    for _ in 0..m {
        let a = tokens.next().unwrap();
        let b = tokens.next().unwrap();
        let c = tokens.next().unwrap();
        d[a - 1][b - 1] = d[a - 1][b - 1].min(&Distance::Finite(c));
        d[b - 1][a - 1] = d[a - 1][b - 1].min(&Distance::Finite(c));
    }
    for k in 0..n {
        for i in 0..n {
            for j in (i+1)..n {
                d[i][j] = d[i][j].min(&d[i][k].add(&d[k][j]));
                d[j][i] = d[i][j];
            }
        }
    }
    let mut ans = String::new();
    for _ in 0..q {
        let a = tokens.next().unwrap();
        let b = tokens.next().unwrap();
        if let Distance::Finite(z) = d[a - 1][b - 1] {
            ans.push_str(&z.to_string());
        } else {
            ans.push_str("-1");
        }
        ans.push('\n');
    }
    println!("{ans}");
}

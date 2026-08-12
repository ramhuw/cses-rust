use std::{io::Read, fmt::Write};


fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();
    let n: usize = tokens.next().unwrap().parse().unwrap();
    let q: usize = tokens.next().unwrap().parse().unwrap();
    let mut x: Vec<u64> = vec![];
    for _ in 0..n {
        x.push(tokens.next().unwrap().parse().unwrap());
    }
    let mut t = SegTree::new(x);
    let mut ans: String = String::new();
    for _ in 0..q {
        if tokens.next().unwrap() == "1" {
            let a: usize = tokens.next().unwrap().parse().unwrap();
            let b: usize = tokens.next().unwrap().parse().unwrap();
            let u: u64 = tokens.next().unwrap().parse().unwrap();
            t.update(a - 1, b, u);
        } else {
            let k: usize = tokens.next().unwrap().parse().unwrap();
            write!(ans, "{}\n", t.query(k - 1)).unwrap();
        }
    }
    println!("{ans}");
}

struct SegTree {
    tree: Vec<Vec<u64>>,
}

impl SegTree {
    fn new(arr: Vec<u64>) -> Self {
        let n = arr.len();
        let mut tree = vec![arr];
        for i in 1usize.. {
            let l: usize = 1 << i;
            if l > n {
                break;
            }
            let mut layer = vec![];
            for j in 0usize.. {
                if l * (j + 1) > n {
                    break;
                }
                layer.push(0);
            }
            tree.push(layer);
        }
        Self { tree }
    }

    fn update(&mut self, start: usize, until: usize, u: u64) {
        if start >= until {
            return;
        }
        let mut i: usize = (start.trailing_zeros() as usize).min(self.tree.len() - 1);
        let mut l = 1 << i;
        while start + l > until {
            l >>= 1;
            i -= 1;
        }
        let j = start / l;
        self.tree[i][j] += u;
        self.update(start + l, until, u);
    }

    fn query(&self, k: usize) -> u64 {
        let mut ans = 0;
        let mut j = k;
        for i in 0..self.tree.len() {
            if j >= self.tree[i].len() {
                break;
            }
            ans += self.tree[i][j];
            j /= 2;
        }
        ans
    }
}

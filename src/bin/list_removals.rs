use std::{fmt::Write, io::Read, ops::Add};

struct SegTree {
    tree: Vec<Vec<usize>>,
}
impl SegTree {
    fn new(n: usize, arr: Vec<usize>) -> Self {
        let mut tree = vec![arr];
        for i in 1.. {
            let l = 1 << i;
            if l > n {
                break;
            }
            let line: Vec<usize> = (0..)
                .take_while(|j| l * j + l <= n)
                .map(|j| tree[i - 1][2 * j].add(tree[i - 1][2 * j + 1]))
                .collect();

            tree.push(line);
        }
        Self { tree }
    }

    fn update(&mut self, i: usize, a: usize) {
        let mut i = i;
        self.tree[0][i] = a;
        for j in 1..self.tree.len() {
            i /= 2;
            if i >= self.tree[j].len() {
                break;
            }

            self.tree[j][i] = self.tree[j - 1][2 * i].add(self.tree[j - 1][2 * i + 1]);
        }
    }

    fn nth(&self, n: usize) -> usize {
        let mut k = 0;
        let mut n = n;
        for i in (0..self.tree.len()).rev() {
            if k >= self.tree[i].len() {
                k *= 2;
            } else if self.tree[i][k] <= n {
                n -= self.tree[i][k];
                k = (k + 1) * 2;
            } else {
                k = k * 2;
            }
        }
        k / 2
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap());
    let n = tokens.next().unwrap();
    let mut t = SegTree::new(n, vec![1; n]);
    let mut x: Vec<usize> = vec![];
    for _ in 0..n {
        x.push(tokens.next().unwrap());
    }
    let mut ans = String::new();
    for p in tokens {
        let i = t.nth(p - 1);
        write!(ans, "{} ", x[i]).unwrap();
        t.update(i, 0);
    }
    println!("{ans}");
}

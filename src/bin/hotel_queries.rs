use std::{fmt::Write, io::Read};

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();
    let n: usize = tokens.next().unwrap().parse().unwrap();
    let m: usize = tokens.next().unwrap().parse().unwrap();
    let mut hotels: Vec<usize> = vec![];
    for _ in 0..n {
        hotels.push(tokens.next().unwrap().parse().unwrap())
    }
    let mut t = SegTree::new(hotels);
    let mut ans = String::new();
    for _ in 0..m {
        let r: usize = tokens.next().unwrap().parse().unwrap();
        if let Some(k) = t.find_first(r) {
            write!(ans, "{} ", k + 1).unwrap();
            t.update(k, t.tree[0][k] - r);
        } else {
            write!(ans, "{} ", 0).unwrap();
        }
    }
    println!("{ans}")
}

struct SegTree {
    tree: Vec<Vec<usize>>,
}

impl SegTree {
    fn new(arr: Vec<usize>) -> Self {
        let n = arr.len();
        let mut tree = vec![arr];
        for i in 1.. {
            let l = 1 << i;
            if l > n {
                break;
            }
            let line: Vec<usize> = (0..)
                .take_while(|j| l * j + l <= n)
                .map(|j| tree[i - 1][2 * j].max(tree[i - 1][2 * j + 1]))
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

            self.tree[j][i] = self.tree[j - 1][2 * i].max(self.tree[j - 1][2 * i + 1]);
        }
    }

    fn find_first(&self, r: usize) -> Option<usize> {
        let mut k = 0usize;
        for i in (0..self.tree.len()).rev() {
            if k >= self.tree[i].len() {
                k *= 2;
            } else if r > self.tree[i][k] {
                k = (k + 1) * 2;
            } else {
                k *= 2;
            }
        }
        k /= 2;
        if k >= self.tree[0].len() {
            None
        } else {
            Some(k)
        }
    }
}

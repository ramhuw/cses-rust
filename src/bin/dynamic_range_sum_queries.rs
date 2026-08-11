use std::fmt::Write;
use std::io::Read;
use std::ops::Add;
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

    fn query(&self, a: usize, b: usize) -> usize {
        let mut a = a;
        let mut ans: usize = 0;
        while a < b {
            let mut k = (a.trailing_zeros() as usize).min(self.tree.len() - 1);
            let mut l = 1 << k;
            while a + l > b {
                l >>= 1;
                k -= 1;
            }
            let i = a / l;
            ans = ans.add(self.tree[k][i]);
            a = a + l;
        }
        ans
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input
        .split_whitespace()
        .map(|a| a.parse::<usize>().unwrap());
    let n: usize = tokens.next().unwrap();
    let q: usize = tokens.next().unwrap();
    let mut x: Vec<usize> = vec![];
    for _ in 0..n {
        x.push(tokens.next().unwrap());
    }
    let mut seg_tree = SegTree::new(n, x);
    let mut ans = String::new();
    for _ in 0..q {
        if tokens.next().unwrap() == 1 {
            let k = tokens.next().unwrap();
            let u = tokens.next().unwrap();
            seg_tree.update(k - 1, u);
        } else {
            let a = tokens.next().unwrap();
            let b = tokens.next().unwrap();
            write!(ans, "{}\n", seg_tree.query(a - 1, b)).unwrap();
        }
    }
    println!("{ans}");
}

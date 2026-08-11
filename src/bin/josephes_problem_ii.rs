use std::fmt::Write;
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
                k *= 2;
            }
        }
        k / 2
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut tokens = input
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap());
    let n = tokens.next().unwrap();
    let k = tokens.next().unwrap();
    let mut p: usize = 0;
    let mut ans: String = String::default();
    let table: Vec<usize> = vec![1; n];
    let mut seg_tree = SegTree::new(n, table);
    for i in (1..=n).rev() {
        p = (p + k) % i;
        let j = seg_tree.nth(p);
        write!(ans, "{} ", j + 1).unwrap();
        seg_tree.update(j, 0);
    }
    println!("{ans}");
}

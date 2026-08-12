use std::{fmt::Write, io::Read};


fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();

}

use std::ops::{Add, Sub};
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

struct FenwickTree<T: Add<Output=T> + Copy + Default> {
    tree: Vec<T>
}

impl<T: Add<Output=T> + Copy + Default> FenwickTree<T> {
    fn new(arr: Vec<T>) -> Self {
        let n = arr.len();
        let mut tree = arr;
        for i in 1..=n {
            let parent = i + (i & i.wrapping_neg());
            if parent <= n {
                tree[parent-1] = tree[parent-1].add(tree[i-1]);
            }
        }
        Self {
            tree
        }
    }

    fn sum(&self, k: usize) -> T {
        let mut i = k;
        let mut s = T::default();
        while i != 0 {
            s = s.add(self.tree[i-1]);
            i -= i & i.wrapping_neg();
        }
        s
    }

    fn add(&mut self, k: usize, a: T) {
        let mut i = k + 1;
        
        let n = self.tree.len();
        while i <= n {
            self.tree[i-1] = self.tree[i-1].add(a);
            i += i & i.wrapping_neg();
        }
    }
}
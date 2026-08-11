use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();
    let n: usize = tokens.next().unwrap().parse().unwrap();
    let hh: Vec<u32> = tokens.map(|h| h.parse::<u32>().unwrap()).collect();
    let mut ii: Vec<usize> = (0..n).collect();
    ii.sort_by(|&i, &j| hh[i].cmp(&hh[j]));
    let mut stack: Vec<usize> = vec![];
    let mut lefts: Vec<Option<usize>> = vec![None; n];
    for i in 0..n {
        while let Some(j) = stack.pop() {
            if hh[j] >= hh[i] {
                stack.push(j);
                break;
            }
        }
        lefts[i] = stack.last().map(|j| *j);
        stack.push(i);
    }
    stack.clear();
    let mut rights: Vec<Option<usize>> = vec![None; n];
    for i in (0..n).rev() {
        while let Some(j) = stack.pop() {
            if hh[j] >= hh[i] {
                stack.push(j);
                break;
            }
        }
        rights[i] = stack.last().map(|j| *j);
        stack.push(i);
    }
    let mut ans: usize = 1;
    let d: Vec<usize> = vec![1; n];
    let mut seg_tree = SegTree::new(n, &d);
    for i in ii {
        let mut v: usize = 1;
        let a = lefts[i].map(|x| x + 1).unwrap_or(0);
        v = v.max(1 + seg_tree.query(a, i));
        let b = rights[i].unwrap_or(n);
        v = v.max(1 + seg_tree.query(i + 1, b));
        seg_tree.update(i, v);
        ans = ans.max(v);
    }
    println!("{ans}");
}

struct SegTree {
    tree: Vec<Vec<usize>>,
}

impl SegTree {
    fn new(n: usize, arr: &[usize]) -> Self {
        let mut tree = vec![arr.to_vec()];
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
            ans = ans.max(self.tree[k][i]);
            a = a + l;
        }
        ans
    }
}

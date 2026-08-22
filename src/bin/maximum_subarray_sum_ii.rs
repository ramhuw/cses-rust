use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();
    let n: usize = tokens.next().unwrap().parse().unwrap();
    let a: usize = tokens.next().unwrap().parse().unwrap();
    let b: usize = tokens.next().unwrap().parse().unwrap();
    let mut prefix_sum = vec![0];
    for i in 0..n {
        let x: i64 = tokens.next().unwrap().parse().unwrap();
        prefix_sum.push(prefix_sum[i] + x);
    }
    let f = SegTree::new(n, prefix_sum);
    let mut ans = i64::MIN;
    for right in a..=n {
        let l1 = if right >= b { right - b } else { 0 };
        let l2 = right - a;
        ans = ans.max(f.tree[0][right] - f.query(l1, l2 + 1));
    }
    println!("{:?}", ans);
}

struct SegTree {
    tree: Vec<Vec<i64>>,
}
impl SegTree {
    fn new(n: usize, arr: Vec<i64>) -> Self {
        let mut tree = vec![arr];
        for i in 1.. {
            let l = 1 << i;
            if l > n {
                break;
            }
            let line: Vec<i64> = (0..)
                .take_while(|j| l * j + l <= n)
                .map(|j| tree[i - 1][2 * j].min(tree[i - 1][2 * j + 1]))
                .collect();

            tree.push(line);
        }
        Self { tree }
    }

    fn query(&self, a: usize, b: usize) -> i64 {
        let mut a = a;
        let mut ans: i64 = i64::MAX;
        while a < b {
            let mut k = (a.trailing_zeros() as usize).min(self.tree.len() - 1);
            let mut l = 1 << k;
            while a + l > b {
                l >>= 1;
                k -= 1;
            }
            let i = a / l;
            ans = ans.min(self.tree[k][i]);
            a = a + l;
        }
        ans
    }
}

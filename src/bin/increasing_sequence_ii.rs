use std::{collections::HashMap, io::Read, ops::Add};
const P: usize = 1000000007;


fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();
    let n: usize = tokens.next().unwrap().parse().unwrap();
    let mut d = vec![0usize; n];
    let mut x: Vec<usize> = vec![];
    for _ in 0..n {
        x.push(tokens.next().unwrap().parse().unwrap());
    }
    let mut y = x.clone();
    y.sort();
    y.dedup();
    let mut t = FenwickTree::new(vec![0usize; y.len()]);
    let mut map: HashMap<usize, usize> = HashMap::new();
    for i in 0..y.len() {
        map.insert(y[i], i);
    }
    for i in 0..n {
        let j = *map.get(&x[i]).unwrap();
        d[i] = 1 + t.sum(j);
        t.add(j, d[i]);
    }
    
    println!("{}", t.sum(t.tree.len()));
}

struct FenwickTree {
    tree: Vec<usize>
}

impl FenwickTree {
    fn new(arr: Vec<usize>) -> Self {
        Self {
            tree: arr
        }
    }

    fn sum(&self, k: usize) -> usize {
        let mut i = k;
        let mut s = 0;
        while i != 0 {
            s = s.add(self.tree[i-1]) % P;
            i -= i & i.wrapping_neg();
        }
        s
    }

    fn add(&mut self, k: usize, a: usize) {
        let mut i = k + 1;
        
        let n = self.tree.len();
        while i <= n {
            self.tree[i-1] = self.tree[i-1].add(a);
            i += i & i.wrapping_neg();
        }
    }
}
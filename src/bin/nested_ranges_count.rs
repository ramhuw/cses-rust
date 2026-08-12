use std::{cmp::Reverse, collections::{BTreeSet, HashMap}, fmt::Write, io::Read, ops::Add};

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();
    let n: usize = tokens.next().unwrap().parse().unwrap();
    let mut ranges: Vec<(u32, u32, usize)> = vec![];
    let mut rights: BTreeSet<u32> = BTreeSet::new();
    for i in 0..n {
        let x: u32 = tokens.next().unwrap().parse().unwrap();
        let y: u32 = tokens.next().unwrap().parse().unwrap();
        ranges.push((x, y, i));
        rights.insert(y);
    }
    let mut ranks: HashMap<u32, usize> = HashMap::new();
    for (i, k) in rights.iter().enumerate() {
        ranks.insert(*k, i);
    }
    ranges.sort_by_key(|&(a, b, c)| (a, Reverse(b), c));
    let mut f = FenwickTree::new(vec![0; ranks.len()]);
    let mut contains = vec![0usize; n];
    for i in (0..n).rev() {
        contains[ranges[i].2] = f.sum(*ranks.get(&ranges[i].1).unwrap() + 1);
        f.add(*ranks.get(&ranges[i].1).unwrap(), 1);
    }
    let mut g = FenwickTree::new(vec![0; ranks.len()]);
    let mut contained = vec![0usize; n];
    for i in 0..n {
        contained[ranges[i].2] = i - g.sum(*ranks.get(&ranges[i].1).unwrap());
        g.add(*ranks.get(&ranges[i].1).unwrap(), 1);
    }
    let mut ans = String::new();
    for c in contains {
        write!(ans, "{} ", c).unwrap();
    }
    ans.push('\n');
    for c in contained {
        write!(ans, "{} ", c).unwrap();
    }
    println!("{ans}");
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
            s = s.add(self.tree[i-1]);
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
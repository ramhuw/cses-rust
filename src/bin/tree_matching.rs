use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap());
    let mut t = Tree::new();
    let n = tokens.next().unwrap();
    for _ in 1..n {
        let a = tokens.next().unwrap();
        let b = tokens.next().unwrap();
        t.update(a, b);
    }
    println!("{}", t.count());
}

struct Node {
    ad: Vec<usize>,
}

impl Node {
    fn new() -> Self {
        Node { ad: vec![] }
    }

    fn push_ad(&mut self, i: usize) {
        self.ad.push(i);
    }
}

struct Tree {
    nodes: Vec<Node>,
    matches1: Vec<usize>,
    matches2: Vec<usize>,
    flags1: Vec<bool>,
    flags2: Vec<bool>
}

impl Tree {
    fn new() -> Self {
        Tree {
            nodes: vec![],
            matches1: vec![],
            matches2: vec![],
            flags1: vec![],
            flags2: vec![]
        }
    }

    fn update(&mut self, a: usize, b: usize) {
        while self.nodes.len() < a || self.nodes.len() < b {
            self.nodes.push(Node::new());
            self.matches1.push(0);
            self.matches2.push(0);
            self.flags1.push(false);
            self.flags2.push(false);
        }
        self.nodes[a-1].push_ad(b-1);
        self.nodes[b-1].push_ad(a-1);
    }

    fn match1(&mut self, i: usize, p: Option<usize>) {
        if self.flags1[i] {
            return;
        }
        let mut sum = 0usize;
        for j in self.nodes[i].ad.clone() {
            if Some(j) != p {
                self.match2(j, Some(i));
                self.match1(j, Some(i));
                sum += self.matches1[j];
            }
        }
        let mut ans = 0usize;
        for j in self.nodes[i].ad.clone() {
            if Some(j) != p {
                ans = ans.max(sum + 1 + self.matches2[j] - self.matches1[j])
            }
        }
        self.matches1[i] = ans;
        self.flags1[i] = true;
    }

    fn match2(&mut self, i: usize, p: Option<usize>) {
        if self.flags2[i] {
            return;
        }
        let mut ans = 0usize;
        for j in self.nodes[i].ad.clone() {
            if Some(j) != p {
                self.match1(j, Some(i));
                ans += self.matches1[j];
            }
        }
        self.matches2[i] = ans;
        self.flags2[i] = true;
    }

    fn count(&mut self) -> usize {
        if self.nodes.is_empty() {
            return 0;
        }
        self.match1(0, None);
        return self.matches1[0];
    }
}

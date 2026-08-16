use std::{cmp::Reverse, io::Read};

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
    radius: Vec<usize>,
    diameter: Vec<usize>
}

impl Tree {
    fn new() -> Self {
        Tree {
            nodes: vec![],
            radius: vec![],
            diameter: vec![]
        }
    }

    fn update(&mut self, a: usize, b: usize) {
        while self.nodes.len() < a || self.nodes.len() < b {
            self.nodes.push(Node::new());
            self.radius.push(0);
            self.diameter.push(0);
        }
        self.nodes[a-1].push_ad(b-1);
        self.nodes[b-1].push_ad(a-1);
    }

    fn deal(&mut self, i: usize, p: Option<usize>) {
        let mut rads: Vec<usize> = vec![];
        for j in self.nodes[i].ad.clone() {
            if Some(j) != p {
                self.deal(j, Some(i));
                rads.push(self.radius[j]);
                self.diameter[i] = self.diameter[i].max(self.diameter[j]).max(self.radius[j] + 1);
                self.radius[i] = self.radius[i].max(1 + self.radius[j]);
            }
        }
        rads.sort_by_key(|&k| Reverse(k));
        if rads.len() >= 2 {
            self.diameter[i] = self.diameter[i].max(2 + rads[0] + rads[1]);
        }
    }

    fn count(&mut self) -> usize {
        if self.nodes.len() == 0 {
            return 0;
        }
        self.deal(0, None);
        return self.diameter[0];
    }
}

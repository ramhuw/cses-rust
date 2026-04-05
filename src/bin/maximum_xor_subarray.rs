use std::io::{self, Read};

#[derive(Clone, Copy)]
struct Node {
    child: [Option<usize>; 2],
}

impl Node {
    fn new() -> Self {
        Self {
            child: [None, None],
        }
    }
}

fn insert(trie: &mut Vec<Node>, value: u32) {
    let mut cur = 0usize;
    for bit in (0..32).rev() {
        let b = ((value >> bit) & 1) as usize;
        let next = match trie[cur].child[b] {
            Some(idx) => idx,
            None => {
                trie.push(Node::new());
                let idx = trie.len() - 1;
                trie[cur].child[b] = Some(idx);
                idx
            }
        };
        cur = next;
    }
}

fn max_xor_with(trie: &[Node], value: u32) -> u32 {
    let mut cur = 0usize;
    let mut best = 0u32;

    for bit in (0..32).rev() {
        let b = ((value >> bit) & 1) as usize;
        let want = b ^ 1;

        if let Some(next) = trie[cur].child[want] {
            best |= 1 << bit;
            cur = next;
        } else if let Some(next) = trie[cur].child[b] {
            cur = next;
        } else {
            break;
        }
    }

    best
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();

    let mut trie = vec![Node::new()];
    insert(&mut trie, 0);

    let mut prefix = 0u32;
    let mut answer = 0u32;

    for _ in 0..n {
        let x: u32 = it.next().unwrap().parse().unwrap();
        prefix ^= x;
        answer = answer.max(max_xor_with(&trie, prefix));
        insert(&mut trie, prefix);
    }

    println!("{}", answer);
}

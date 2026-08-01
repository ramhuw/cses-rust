use std::io::Read;
const P: usize = 1000000007;

struct Node {
    nexts: [Option<usize>; 26],
    terminal: bool
}

impl Node {
    fn new() -> Self {
        Node { nexts: [None; 26], terminal: false }
    }
}

struct Trie {
    nodes: Vec<Node>,
}

impl Trie {
    fn new() -> Self {
        Self {
            nodes: vec![Node::new()],
        }
    }

    fn insert(&mut self, word: &str) {
        let mut current_index = 0;
        for c in word.chars() {
            let next_value = c as usize - 'a' as usize;
            if let Some(next_index) = self.nodes[current_index].nexts[next_value] {
                current_index = next_index
            } else {
                let new_node = Node::new();
                let new_index = self.nodes.len();
                self.nodes.push(new_node);
                self.nodes[current_index].nexts[next_value] = Some(new_index);
                current_index = new_index;
            }
        }
        self.nodes[current_index].terminal = true;
    }

    fn occurs(&self, sentense: &str) -> Vec<usize> {
        let mut current_index = 0;
        let mut ans = Vec::new();
        let mut step: usize = 0;
        for c in sentense.chars() {
            step += 1;
            let next_value = c as usize - 'a' as usize;
            if let Some(new_index) = self.nodes[current_index].nexts[next_value] {
                current_index = new_index;
                if self.nodes[current_index].terminal {
                    ans.push(step);
                }
            } else {
                break;
            }
        }
        ans
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let sentense = lines.next().unwrap();
    let n = sentense.len();
    let _ = lines.next().unwrap().parse::<usize>().unwrap();
    let mut trie = Trie::new();
    for line in lines {
        trie.insert(line);
    }
    let mut d: Vec<usize> = vec![0; n + 1];
    d[0] = 1;
    for i in 0..n {
        for l in trie.occurs(&sentense[i..]) {
            d[i + l] = (d[i + l] + d[i]) % P;
        }
    }
    println!("{}", d[n]);
}

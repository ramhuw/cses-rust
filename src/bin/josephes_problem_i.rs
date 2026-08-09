fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();
    let mut nodes: Vec<Node> = (0..n).map(
        |i| Node {
            val: i,
            next: (i + 1) % n
        }
    ).collect();
    let mut current = 0;
    let mut ans = String::new();
    for _ in 0..n {
        let next = nodes[current].next;
        ans.push_str(&format!("{} ", nodes[next].val + 1));
        let new_next = nodes[next].next;
        nodes[current].next = new_next;
        current = new_next;
    }
    println!("{ans}");
}

struct Node {
    val: usize,
    next: usize
}
fn main() {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).unwrap();
    let n: u32 = n.trim().parse().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    use std::collections::HashSet;
    let mut HashSet: HashSet<u64> = HashSet::new();
    for i in input.split_whitespace() {
        HashSet.insert(i.parse().unwrap());
    }
    println!("{}", HashSet.len());
}

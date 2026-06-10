fn main() {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).unwrap();
    let _: u32 = n.trim().parse().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    use std::collections::HashSet;
    let mut set: HashSet<u64> = HashSet::new();
    for i in input.split_whitespace() {
        set.insert(i.parse().unwrap());
    }
    println!("{}", set.len());
}

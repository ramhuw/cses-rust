use std::io::Read;
fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let _: usize = lines.next().unwrap().parse().unwrap();
    let mut xx: Vec<u64> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    xx.sort();
    xx.reverse();
    let mut target: u64 = 1;
    while let Some(x) = xx.pop() {
        if x > target {
            break;
        } else {
            target = target + x;
        }
    }
    println!("{target}");
}
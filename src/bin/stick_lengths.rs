use std::io::Read;
fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let n: usize = lines.next().unwrap().parse().unwrap();
    let mut pp: Vec<u64> = lines.next().unwrap().split_whitespace().map(|x| x.parse().unwrap()).collect();
    pp.sort();
    let target = pp[n/2];
    let ans: u64 = pp.iter().map(|p| if p >= &target {p - target} else {target - p}).sum();
    println!("{}", ans);
}
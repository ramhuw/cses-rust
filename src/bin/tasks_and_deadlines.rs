use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();
    let n: usize = tokens.next().unwrap().parse().unwrap();
    let mut ans: i64 = 0;
    let mut tasks: Vec<(i64, i64)> = vec![];
    for _ in 0..n {
        let a: i64 = tokens.next().unwrap().parse().unwrap();
        let d: i64 = tokens.next().unwrap().parse().unwrap();
        tasks.push((a, d));
    }
    tasks.sort();
    for i in 0..n {
        ans += tasks[i].1 - tasks[i].0 * (n - i) as i64;
    }
    println!("{ans}");
}
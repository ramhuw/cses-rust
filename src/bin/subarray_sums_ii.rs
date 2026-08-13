use std::{collections::HashMap, io::Read};


fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();
    let n: usize = tokens.next().unwrap().parse().unwrap();
    let x: i64 = tokens.next().unwrap().parse().unwrap();
    let mut ss: HashMap<i64, usize> = HashMap::new();
    ss.insert(0, 1);
    let mut ans: usize = 0;
    let mut s: i64 = 0;
    for _ in 0..n {
        let a: i64 = tokens.next().unwrap().parse().unwrap();
        s += a;
        if ss.contains_key(&(s - x)) {
            ans += ss.get(&(s-x)).unwrap();
        }
        *ss.entry(s).or_insert(0) += 1;
    }
    println!("{ans}");
}
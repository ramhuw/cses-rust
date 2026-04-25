use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let arr: Vec<usize> = (1..n)
        .map(|_| iter.next().unwrap().parse().unwrap())
        .collect();
    let mut children: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut subs: Vec<usize> = vec![0; n];
    for (i, &a) in arr.iter().enumerate() {
        children[a - 1].push(i + 1);
    }
    let mut flags = vec![true; n];
    let mut stack = vec![0];
    while let Some(person) = &stack.pop() {
        if flags[*person] {
            stack.push(*person);
            flags[*person] = false;
            for &child in &children[*person] {
                stack.push(child);
            }
        } else {
            subs[*person] = children[*person].iter().map(|s| 1 + subs[*s]).sum();
        }
    }
    println!(
        "{}",
        subs.iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}

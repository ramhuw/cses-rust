use std::{io::Read, fmt::Write};

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();
    let n: usize = tokens.next().unwrap().parse().unwrap();
    let mut xs: Vec<u32> = vec![];
    let mut stack: Vec<usize> = vec![];
    let mut ans = String::new();
    for i in 0..n {
        let x = tokens.next().unwrap().parse().unwrap();
        xs.push(x);
        while let Some(j) = stack.last() {
            if xs[*j] >= x {
                stack.pop();
            } else {
                break;
            }
        }
        if let Some(j) = stack.last() {
            write!(ans, "{} ", j + 1).unwrap();
        } else {
            write!(ans, "{} ", 0).unwrap()
        }
        stack.push(i);
    }
    println!("{ans}");
}
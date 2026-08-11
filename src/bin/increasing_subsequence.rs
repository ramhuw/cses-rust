use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();
    let n: usize = tokens.next().unwrap().parse().unwrap();
    let mut d: Vec<u32> = vec![0];
    for _ in 0..n {
        let x: u32 = tokens.next().unwrap().parse().unwrap();
        let mut left: usize = 0;
        let mut right: usize = d.len();
        while left + 1 < right {
            let middle = (left + right) / 2;
            if d[middle] < x {
                left = middle;
            } else {
                right = middle;
            }
        }
        if right == d.len() {
            d.push(x);
        } else {
            d[right] = x;
        }
    }
    println!("{}", d.len() - 1);
}

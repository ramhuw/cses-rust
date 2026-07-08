use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.trim().lines();
    let mut line1 = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap());
    let n = line1.next().unwrap();
    let x = line1.next().unwrap();
    let mut ps = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    ps.sort();
    let mut count: u32 = 0;
    let mut left = 0;
    let mut right = n.saturating_sub(1);
    while left <= right {
        if left == right || ps[left as usize] + ps[right as usize] <= x {
            left = left.saturating_add(1);
            right = right.saturating_sub(1);
        } else {
            right = right.saturating_sub(1);
        }
        count += 1;
    }
    println!("{}", count);
}

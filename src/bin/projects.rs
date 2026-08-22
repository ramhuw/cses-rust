use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();
    let n: usize = tokens.next().unwrap().parse().unwrap();
    let mut project: Vec<(u32, u32, u32)> = vec![(0, 0, 0)];
    for _ in 0..n {
        let a: u32 = tokens.next().unwrap().parse().unwrap();
        let b: u32 = tokens.next().unwrap().parse().unwrap();
        let c: u32 = tokens.next().unwrap().parse().unwrap();
        project.push((a, b, c));
    }
    project.sort_unstable_by_key(|&(_, b, _)| b);
    let mut d: Vec<u64> = vec![0; n + 1];
    for i in 1..=n {
        d[i] = d[i - 1];
        let mut left = 0usize;
        let mut right = i - 1;
        while left < right {
            let middle = (left + right + 1) / 2;
            if project[middle].1 < project[i].0 {
                left = middle;
            } else {
                right = middle - 1;
            }
        }
        d[i] = d[i].max(d[left] + project[i].2 as u64);
    }
    println!("{}", d[n]);
}

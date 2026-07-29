use std::collections::HashMap;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.clear();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut xs: Vec<u32> = input
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    xs.sort();
    xs.reverse();
    let mut d: u32 = 1000000;
    let mut frequencies: HashMap<u32, usize> = HashMap::new();
    for &x in &xs {
        *frequencies.entry(x).or_insert(0) += 1;
    }
    loop {
        let mut count = 0;
        let mut dd = d;
        while dd <= 1000000 {
            count += frequencies.get(&dd).unwrap_or(&0);
            dd += d;
            if count >= 2 {
                println!("{d}");
                return;
            }
        }
        d -= 1;
    }
}

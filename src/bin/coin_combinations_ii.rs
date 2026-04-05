use std::io::stdin;
fn main() {
    let mut input1: String = String::new();
    let mut input2: String = String::new();
    stdin().read_line(&mut input1).unwrap();
    stdin().read_line(&mut input2).unwrap();
    let mut iter1 = input1
        .trim()
        .split_whitespace()
        .map(|a| a.to_string().parse::<usize>().unwrap());
    let iter2 = input2
        .trim()
        .split_whitespace()
        .map(|a| a.to_string().parse::<usize>().unwrap());
    let _n = iter1.next().unwrap();
    let x = iter1.next().unwrap();
    let mut coins: Vec<usize> = iter2.clone().collect();
    coins.sort();
    let max_coin = iter2.max().unwrap();
    let mut space: Vec<usize> = vec![0; x.max(max_coin) + 1];
    space[0] = 1;
    for &coin in &coins {
        for i in 0..=x {
            if i + coin < space.len() {
                space[i + coin] += space[i];
                space[i + coin] %= 10u32.pow(9) as usize + 7;
            }
        }
    }
    println!("{:?}", space[x]);
}

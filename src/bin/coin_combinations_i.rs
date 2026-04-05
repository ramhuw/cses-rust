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
    let mut iter2 = input2
        .trim()
        .split_whitespace()
        .map(|a| a.to_string().parse::<usize>().unwrap());
    let _n = iter1.next().unwrap();
    let x = iter1.next().unwrap();
    let coins: Vec<usize> = iter2.clone().collect();
    let mut space: Vec<usize> = vec![0; (x + 1).max(iter2.clone().max().unwrap() + 1)];
    while let Some(index) = iter2.next() {
        space[index] = 1;
    }
    for i in 0..=x {
        for &coin in &coins {
            if i >= coin {
                space[i] = (space[i] + space[i - coin]) % (10u32.pow(9u32) as usize + 7);
            }
        }
    }
    println!("{}", space[x]);
}

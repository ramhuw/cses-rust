fn main() {
    use std::io;
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read line");
    let n: i32 = n.trim().parse().expect("Please type a number!");
    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("Failed to read line");
    let mut x = x.trim().split_whitespace()
        .map(|s| s.parse::<i64>().expect("Parse error"))
        .collect::<Vec<i64>>();
    let mut counter = 0;
    for i in 1..n as usize {
        if x[i] < x[i-1] {
            counter += x[i-1] - x[i];
            x[i] = x[i-1];
        }
    }
    println!("{}", counter);
}

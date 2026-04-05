fn main() {
    use std::io;
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read line");
    let n: u128 = n.trim().parse().expect("Please type a number!");
    let mut v: Vec<u128> = Vec::new();
    let mut k = n;
    while k > 0 {
        v.push(k % 5);
        k = (k - k % 5) / 5;
    }
    println!("{}", (n - v.iter().sum::<u128>())/4);
}
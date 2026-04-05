fn main() {
    use std::io;
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read line");
    let mut n: i32 = n.trim().parse().expect("Please type a number!");
    n = n % 1000000006;
    let mut ans = 1;
    for _ in 0..n {
        ans = (ans * 2) % 1000000007;
    }
    println!("{}", ans);
}

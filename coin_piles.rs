fn main() {
    use std::io;
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read line");
    let n: u128 = n.trim().parse().expect("Please type a number!");
    let mut v: Vec<String> = Vec::new();
    for _ in 0..n {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Failed to read line");
        let mut iter = line.split_whitespace();
        let m: i64 = iter.next().unwrap().parse().expect("Please type a number!");
        let n: i64 = iter.next().unwrap().parse().expect("Please type a number!");
        if (m - 2 * n) % 3 == 0 && (2 * m - n) % 3 == 0 && m - 2 * n <= 0 && 2 * m - n >= 0 {
            v.push("YES\n".to_string());
        } else {
            v.push("NO\n".to_string());
        }

    }
    for answer in v {
        print!("{}", answer);
    }
}

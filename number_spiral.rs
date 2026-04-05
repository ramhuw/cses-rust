fn main() {
    use std::io;
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read line");
    let n: i32 = n.trim().parse().expect("Please type a number!");
    let mut result: Vec<String> = Vec::new();
    for i in 0..n {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Failed to read line");
        let mut iter = line.split_whitespace();
        let x: i64 = iter.next().unwrap().parse().expect("Please type a number!");
        let y: i64 = iter.next().unwrap().parse().expect("Please type a number!");
        result.push(number_spiral(x, y).to_string());
    }
    println!("{}", result.join("\n"));

}

fn number_spiral(x: i64, y:i64) -> i64 {
    let mut ans: i64;
    if x > y {
        if x % 2 == 1 {
            ans = (x - 1) * (x - 1) + y;
        } else {
            ans = x * x - y + 1;
        }
    } else {
        if y % 2 == 1 {
            ans = y * y - x + 1;
        } else {
            ans = (y - 1) * (y - 1) + x;
        }
    }
    ans
}
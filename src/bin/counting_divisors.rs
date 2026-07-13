use std::io::Read;

fn count(x: u32) -> u32 {
    let mut ans: u32 = 0;
    let mut i = 1;
    while i*i<x {
        if x % i == 0 {
            ans += 2;
        }
        i += 1;
    }
    if i * i == x {
        ans += 1;
    }
    ans
}
fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let _ = lines.next().unwrap();
    let xs = lines.map(|x| x.parse::<u32>().unwrap());
    let ans = xs.map(count).map(|z| z.to_string()).reduce(|acc, a| acc + "\n" + &a).unwrap();
    println!("{ans}");
}

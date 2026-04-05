fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();
    let mut ans: Vec<u32> = vec![1, 2, 4, 8, 16, 32];
    for i in 6..n {
        let mut x: u32 = 0;
        for k in 1..=6 {
            x = (x + ans[i - k]) % (10u32.pow(9) + 7);
        }

        ans.push(x);
    }
    println!("{}", ans[n - 1]);
}

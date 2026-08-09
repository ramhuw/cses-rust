fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut tokens = input.split_whitespace().map(|x| x.parse::<usize>().unwrap());
    let n = tokens.next().unwrap();
    let a = tokens.next().unwrap();
    let b = tokens.next().unwrap();
    let mut ans: f64 = 0.0;
    let mut d: Vec<Vec<f64>> = vec![(0..b).map(|i| if i < 6 {1.0/6.0} else {0.0}).collect()];
    for i in 1..n {
        d.push(Vec::new());
        for j in 0..b {
            d[i].push(0.0);
            for k in 1..=6 {
                if j >= k {
                    d[i][j] += d[i-1][j-k] * 1.0/6.0;
                }
            }
        }
    }
    for j in a..=b {
        ans += d[n-1][j-1];
    }
    println!("{:.6}", ans);
}

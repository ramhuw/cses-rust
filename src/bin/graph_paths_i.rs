use std::io::Read;
const P: usize = 1000000007;
fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace().map(|x| x.parse::<usize>().unwrap());
    let n = tokens.next().unwrap();
    let m = tokens.next().unwrap();
    let mut k = tokens.next().unwrap();
    let mut ad: Vec<Vec<usize>> = vec![vec![0; n]; n];
    for _ in 0..m {
        let a = tokens.next().unwrap();
        let b = tokens.next().unwrap();
        ad[a-1][b-1] += 1;
    }
    let mut ans: Vec<Vec<usize>> = (0..n).map(|i| (0..n).map(|j| if j == i {1} else {0}).collect()).collect();
    while k != 0 {
        if k & 1 == 1 {
            ans = mul(&ans, &ad, n);
        }
        ad = mul(&ad, &ad, n);
        k >>= 1;
    }
    println!("{}", ans[0][n-1]);
}

fn mul(x: &Vec<Vec<usize>>, y: &Vec<Vec<usize>>, n: usize) -> Vec<Vec<usize>> {
    let mut ans = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                ans[i][j] += x[i][k] * y[k][j] % P;
                ans[i][j] %= P;
            }
        }
    }
    ans
}
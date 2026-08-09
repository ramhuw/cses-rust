use std::io::Read;
fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace().map(|x| x.parse::<usize>().unwrap());
    let n = tokens.next().unwrap();
    let m = tokens.next().unwrap();
    let mut k = tokens.next().unwrap();
    let mut ad: Vec<Vec<Option<usize>>> = vec![vec![None; n]; n];
    for _ in 0..m {
        let a = tokens.next().unwrap();
        let b = tokens.next().unwrap();
        let c = tokens.next().unwrap();
        if ad[a-1][b-1].is_none() || ad[a-1][b-1].unwrap() > c {
            ad[a-1][b-1] = Some(c);
        }
    }
    let mut ans: Vec<Vec<Option<usize>>> = (0..n).map(|i| (0..n).map(|j| if j == i {Some(0)} else {None}).collect()).collect();
    while k != 0 {
        if k & 1 == 1 {
            ans = mul(&ans, &ad, n);
        }
        ad = mul(&ad, &ad, n);
        k >>= 1;
    }
    println!("{}", if let Some(l) = ans[0][n-1] {l.to_string()} else {"-1".to_string()});
}

fn mul(x: &Vec<Vec<Option<usize>>>, y: &Vec<Vec<Option<usize>>>, n: usize) -> Vec<Vec<Option<usize>>> {
    let mut ans = vec![vec![None; n]; n];
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                if x[i][k].is_some() && y[k][j].is_some() && (ans[i][j].is_none() || ans[i][j].unwrap() > x[i][k].unwrap() + y[k][j].unwrap()) {
                    ans[i][j] = Some(x[i][k].unwrap() + y[k][j].unwrap());
                }
            }
        }
    }
    ans
}
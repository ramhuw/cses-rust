const P: usize = 1000000007;
fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();
    let mut d: Vec<usize> = vec![1];
    for i in 1..=6 {
        d.push(0);
        for j in 1..=i {
            d[i] += d[i-j];
        }
    }
    if n <= 6 {
        println!("{}", d[n]);
        return ;
    }
    let mut a: Vec<Vec<usize>> = vec![
        vec![0, 1, 0, 0, 0, 0],
        vec![0, 0, 1, 0, 0, 0],
        vec![0, 0, 0, 1, 0, 0],
        vec![0, 0, 0, 0, 1, 0],
        vec![0, 0, 0, 0, 0, 1],
        vec![1, 1, 1, 1, 1, 1]
    ];
    let mut b: Vec<Vec<usize>> = vec![
        vec![1, 0, 0, 0, 0, 0],
        vec![0, 1, 0, 0, 0, 0],
        vec![0, 0, 1, 0, 0, 0],
        vec![0, 0, 0, 1, 0, 0],
        vec![0, 0, 0, 0, 1, 0],
        vec![0, 0, 0, 0, 0, 1]
    ];
    let mut k = n - 6;
    while k != 0 {
        if k & 1 == 1 {
            b = mul(&a, &b);
        }
        a = mul(&a, &a);
        k >>= 1;
    }
    let mut ans: usize = 0;
    for i in 0..6 {
        ans += b[5][i] * d[i+1] % P;
        ans %= P;
    }
    println!("{ans}");
}

fn mul(x: &Vec<Vec<usize>>, y: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut ans: Vec<Vec<usize>> = vec![vec![0; 6]; 6];
    for i in 0..6 {
        for j in 0..6 {
            for k in 0..6 {
                ans[i][j] += x[i][k] * y[k][j] % P;
                ans[i][j] %= P;
            }
        }
    }
    ans
}
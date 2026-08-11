use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();
    let n = tokens.next().unwrap().parse::<usize>().unwrap();
    let xx: Vec<i64> = tokens.map(|x| x.parse::<i64>().unwrap()).collect();
    let mut d: Vec<Vec<i64>> = vec![vec![0; n + 1]; n + 1];
    let mut e: Vec<Vec<i64>> = vec![vec![0; n + 1]; n + 1];
    for l in 1..=n {
        for start in 0..=(n - l) {
            let flag = xx[start] + e[start + 1][l - 1] >= xx[start + l - 1] + e[start][l - 1];
            d[start][l] =
                (xx[start] + e[start + 1][l - 1]).max(xx[start + l - 1] + e[start][l - 1]);
            e[start][l] = if flag {
                d[start + 1][l - 1]
            } else {
                d[start][l - 1]
            }
        }
    }
    println!("{:?}", d[0][n]);
}

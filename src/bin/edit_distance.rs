fn main() {
    let mut x = String::new();
    std::io::stdin().read_line(&mut x).unwrap();
    let mut y = String::new();
    std::io::stdin().read_line(&mut y).unwrap();
    let a: Vec<char> = x.trim().chars().collect();
    let b: Vec<char> = y.trim().chars().collect();
    let n = a.len();
    let m = b.len();
    let mut d: Vec<Vec<usize>> = vec![vec![0; m + 1]; n + 1];
    for i in 0..=n {
        for j in 0..=m {
            if i == 0 || j == 0 {
                d[i][j] = i + j;
            } else if a[i-1] == b[j-1] {
                d[i][j] = d[i-1][j-1];
            } else {
                d[i][j] = d[i-1][j-1].min(d[i-1][j]).min(d[i][j-1]) + 1
            }
        }
    }
    println!("{}", d[n][m]);
}
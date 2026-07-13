use std::io::Read;

const P: usize = 1000000007;
fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let n: usize = lines.next().unwrap().parse().unwrap();
    let mut grid: Vec<Vec<char>> = vec![];
    while let Some(line) = lines.next() {
        grid.push(line.chars().collect());
    }
    let mut ans: Vec<Vec<usize>> = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            if grid[i][j] == '*' {
                continue;
            }
            if (i, j) == (0, 0) {
                ans[i][j] += 1;
            }
            if i > 0 {
                ans[i][j] += ans[i-1][j];
                ans[i][j] %= P;
            }
            if j > 0 {
                ans[i][j] += ans[i][j-1];
                ans[i][j] %= P;
            }
        }
    }
    println!("{}", ans[n-1][n-1]);
}
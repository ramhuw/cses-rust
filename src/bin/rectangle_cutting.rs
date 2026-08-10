fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut tokens = input.split_whitespace();
    let a: usize = tokens.next().unwrap().parse().unwrap();
    let b: usize = tokens.next().unwrap().parse().unwrap();
    let mut d: Vec<Vec<usize>> = vec![vec![usize::MAX; a.max(b) + 1]; a.max(b) + 1];
    for i in 0..=a.max(b) {
        for j in i..=a.max(b) {
            if i == 0 || i == j {
                d[i][j] = 0;
            } else {
                for k in 1..i {
                    d[i][j] = d[i][j].min(d[k][j].saturating_add(d[i - k][j]).saturating_add(1));
                }
                for l in 1..j {
                    d[i][j] = d[i][j].min(d[i][l].saturating_add(d[i][j - l]).saturating_add(1));
                }
            }
            d[j][i] = d[i][j];
        }
    }
    println!("{}", d[a][b]);
}

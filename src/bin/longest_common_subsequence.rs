use std::io::Read;
 
fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace().map(|a| a.parse::<usize>().unwrap());
    let n = tokens.next().unwrap();
    let m = tokens.next().unwrap();
    let mut a: Vec<usize> = Vec::new();
    for _ in 0..n {
        a.push(tokens.next().unwrap());
    }
    let mut b: Vec<usize> = Vec::new();
    for _ in 0..m {
        b.push(tokens.next().unwrap());
    }
    let mut d: Vec<Vec<usize>> = vec![vec![0; m+1]; n+1];
    for i in 0..n {
        for j in 0..m {
            if a[i] == b[j] {
                d[i+1][j+1] = d[i][j] + 1;
            } else {
                
                d[i+1][j+1] = d[i][j+1].max(d[i+1][j]);
            }
        }
    }
    let mut ans = Vec::new();
    let mut i = n - 1;
    let mut j = m - 1;
    loop {
        if a[i] == b[j] {
            ans.push(a[i]);
            if i == 0 || j == 0 {
                break;
            }
            i -= 1;
            j -= 1;
        } else {
            if d[i+1][j] > d[i][j+1] {
                if j == 0 {
                    break;
                }
                j -= 1;
            } else {
                if i == 0 {
                    break;
                }
                i -= 1;
            }
        }
    }
    println!("{}\n{}", d[n][m], ans.iter().rev().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
}

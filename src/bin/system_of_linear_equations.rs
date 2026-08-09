use std::io::Read;
const P: usize = 1000000007;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap());
    let n = tokens.next().unwrap();
    let m = tokens.next().unwrap();
    let mut a: Vec<Vec<usize>> = Vec::new();
    let mut b: Vec<usize> = Vec::new();
    let mut x: Vec<usize> = vec![0; m];
    for _ in 0..n {
        let mut aa: Vec<usize> = Vec::new();
        for _ in 0..m {
            aa.push(tokens.next().unwrap());
        }
        a.push(aa);
        b.push(tokens.next().unwrap());
    }
    
    let mut i = 0;
    let mut j = 0;
    while i < n && j < m {
        let mut ii = i;
        while ii < n && a[ii][j] == 0 {
            ii += 1;
        }
        if ii < n && a[ii][j] != 0 {
            if ii != i {
                for jj in 0..m {
                    (a[i][jj], a[ii][jj]) = (a[ii][jj], a[i][jj]);
                }
                (b[i], b[ii]) = (b[ii], b[i]);
            }
            let d = inv(a[i][j]);
            for jj in 0..m {
                a[i][jj] *= d;
                a[i][jj] %= P;
            }
            b[i] *= d;
            b[i] %= P;
            for k in 0..n {
                if k != i {
                    let e = a[k][j];
                    for jj in 0..m {
                        a[k][jj] += P - e * a[i][jj] % P;
                        a[k][jj] %= P;
                    }
                    b[k] += P - e * b[i] % P;
                    b[k] %= P;
                }
            }
            i += 1;
            j += 1;
        } else {
            j += 1;
        }
    }
    let mut i = 0;
    let mut j = 0;
    while i < n {
        let k = j;
        while j < m && a[i][j] == 0 {
            j += 1;
        }
        if j < m && a[i][j] != 0 {
            x[j] = b[i];
            i += 1;
            j += 1;
        } else {
            if b[i] != 0 {
                println!("-1");
                return;
            }
            i += 1;
            j = k;
        }
    }

    let ans = x
        .iter()
        .map(|xi| xi.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    println!("{ans}");
}

fn exp(a: usize, b: usize) -> usize {
    let mut ans = 1;
    let mut x = a;
    let mut y = b % (P - 1);
    while y != 0 {
        if y & 1 == 1 {
            ans = (ans * x) % P;
        }
        x = (x * x) % P;
        y >>= 1;
    }
    ans
}

fn inv(a: usize) -> usize {
    exp(a, P - 2)
}

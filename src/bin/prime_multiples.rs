fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut line1 = input.trim().split_whitespace().map(|x| x.parse::<usize>().unwrap());
    let n = line1.next().unwrap();
    let k = line1.next().unwrap();
    input.clear();
    std::io::stdin().read_line(&mut input).unwrap();
    let aa: Vec<usize> = input.trim().split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect();
    let mut ans = 0usize;
    'outer: for i in 1..(1 << k) {
        let mut p: usize = 1;
        let mut count: usize = 0;
        for j in 0..k {
            if (1 << j) & i != 0 {
                count += 1;
                p = p.checked_mul(aa[j]).unwrap_or(n + 1);
                if p > n {
                    continue 'outer;
                }
            }
        }
        let atr = n / p;
        if count % 2 == 1 {
            ans += atr;
        } else {
            ans -= atr;
        }
    }
    println!("{}", ans);
}
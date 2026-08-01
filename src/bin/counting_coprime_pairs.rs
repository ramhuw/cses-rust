use std::collections::HashMap;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.clear();
    std::io::stdin().read_line(&mut input).unwrap();
    let xs = input
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap());
    let mut frequencies: HashMap<usize, usize> = HashMap::new();
    let mut m = 1;
    for x in xs {
        m = m.max(x);
        *frequencies.entry(x).or_insert(0) += 1;
    }
    let mut c: Vec<usize> = vec![0; m + 1];
    let mut e: Vec<usize> = vec![0; m + 1];
    for d in (1..=m).rev() {
        for b in (d..=m).step_by(d) {
            if let Some(k) = frequencies.get(&b) {
                c[d] += k;
            }
        }
        if c[d] >= 2 {
            e[d] += c[d] * (c[d] - 1) / 2;
            for b in (2 * d..=m).step_by(d) {
                e[d] -= e[b];
            }
        }
    }
    println!("{}", e[1]);
}

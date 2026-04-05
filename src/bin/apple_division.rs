fn main() {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).expect("Failed to read line");
    let n: usize = n.trim().parse().unwrap();

    let mut p = String::new();
    std::io::stdin().read_line(&mut p).expect("Failed to read line");
    let p: Vec<i64> = p.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();

    let total: i64 = p.iter().sum();
    let mut best = total; // worst case: all in one group

    for mask in 0..(1u32 << n) {
        let mut group_sum: i64 = 0;
        for i in 0..n {
            if mask & (1 << i) != 0 {
                group_sum += p[i];
            }
        }
        let diff = (total - 2 * group_sum).abs();
        best = best.min(diff);
    }

    println!("{}", best);
}


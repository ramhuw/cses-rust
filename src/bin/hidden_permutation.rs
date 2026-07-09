fn main() {
    use std::cmp::Ordering;
    use std::io;
    let mut ninput = String::new();
    io::stdin().read_line(&mut ninput).unwrap();
    let n = ninput.trim().parse::<usize>().unwrap();
    let mut indeces: Vec<usize> = (1..=n).collect();
    indeces.sort_by(|a, b| {
        if a == b {
            return Ordering::Equal;
        }
        println!("? {a} {b}");
        let mut ans = String::new();
        io::stdin().read_line(&mut ans).unwrap();
        if ans.trim() == "YES" {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });
    let mut ans = vec![0 as usize; n];
    for i in 0..n {
        ans[indeces[i]-1] = i + 1;
    }
    println!(
        "! {}",
        ans.iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}

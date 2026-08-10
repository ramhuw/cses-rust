use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace().map(|x| x.parse::<usize>().unwrap());
    let _ = tokens.next().unwrap();
    let mut d = [false; 100001];
    d[0] = true;
    for x in tokens {
        for i in (0..=(100000-x)).rev() {
            if d[i] {
                d[i + x] = true;
            }
        }
    }
    let mut count = 0usize;
    let mut ans = String::new();
    for i in 1..=100000 {
        if d[i] {
            count += 1;
            ans.push_str(&format!("{} ", i));
        }
    }
    println!("{count}\n{ans}");
}
use std::io::Read;
const P: usize = 1000000007;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace().map(|x| x.parse::<usize>().unwrap());
    let t = tokens.next().unwrap();
    let mut a: Vec<usize> = vec![1];
    let mut b: Vec<usize> = vec![1];
    let mut h = 1usize;
    let mut ans = String::new();
    for _ in 0..t {
        let n = tokens.next().unwrap();
        while h < n {
            a.push((4 * a[h-1] % P + b[h-1]) % P);
            b.push((a[h-1] + 2 * b[h-1] % P) % P);
            h += 1;
        }
        ans.push_str(&format!("{}\n", (a[n-1] + b[n-1]) % P))
    }
    println!("{ans}");
}
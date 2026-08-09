use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace().map(|x| x.parse::<usize>().unwrap());
    let _ = tokens.next().unwrap();
    let x = tokens.next().unwrap();
    let mut a: Vec<(usize, usize)> = tokens.enumerate().map(|(i, j)| (j, i+1)).collect();
    a.sort();
    for i in 0..a.len() {
        if a[i].0 * 3 > x {
            break;
        }
        let target = x - a[i].0;
        let mut j = i + 1;
        let mut k = a.len() - 1;
        while j < k {
            let s = a[j].0 + a[k].0;
            if s == target {
                println!("{} {} {}", a[i].1, a[j].1, a[k].1);
                return ;
            } else if s < target {
                j += 1;
            } else {
                k -= 1;
            }
        }
    }
    println!("IMPOSSIBLE");
}
use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace().map(|x| x.parse::<usize>().unwrap());
    let n = tokens.next().unwrap();
    let x = tokens.next().unwrap();
    let a: Vec<usize> = tokens.collect();
    let mut left = 0;
    let mut right = 0;
    let mut s = 0;
    let mut ans = 0;
    while left <= right {
        if s < x {
            if right >= n {
                break;
            }
            s += a[right];
            right += 1;
        } else if s >= x {
            if s == x {
                ans += 1;
            }
            if left >= n {
                break;
            }
            s -= a[left];
            left += 1;
        }
    }
    println!("{ans}");
}
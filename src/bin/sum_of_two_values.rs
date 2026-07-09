use std::io::Read;
fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let mut line1 = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap());
    let n = line1.next().unwrap() as usize;
    let x = line1.next().unwrap();
    let mut aa = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .enumerate()
        .collect::<Vec<(usize, u32)>>();
    aa.sort_by(|a, b| a.1.cmp(&b.1));
    let mut i = 0 as usize;
    let mut j = n - 1;
    let mut ans: Option<(usize, usize)> = None;
    while i < j {
        let temp = aa[i].1 + aa[j].1;
        if temp == x {
            ans = Some((aa[i].0 + 1, aa[j].0 + 1));
            break;
        } else if temp > x {
            j = j.saturating_sub(1);
        } else {
            i = i.saturating_add(1);
        }
    }
    if let Some((a1, a2)) = ans {
        println!("{} {}", a1, a2);
    } else {
        println!("IMPOSSIBLE");
    }
}

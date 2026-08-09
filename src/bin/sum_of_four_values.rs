use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap());
    let _ = tokens.next().unwrap();
    let x = tokens.next().unwrap();
    let mut a: Vec<(usize, usize)> = tokens.enumerate().map(|(i, j)| (j, i + 1)).collect();
    a.sort();
    for i in 0..a.len() {
        if a[i].0 * 4 > x {
            break;
        }
        for j in (i + 1)..a.len() {
            if a[i].0 + 3 * a[j].0 > x {
                break;
            }
            let target = x - a[i].0 - a[j].0;
            let mut k = j + 1;
            let mut l = a.len() - 1;
            while k < l {
                let s = a[k].0 + a[l].0;
                if s == target {
                    println!("{} {} {} {}", a[i].1, a[j].1, a[k].1, a[l].1);
                    return;
                } else if s < target {
                    let mut left = k + 1;
                    let mut right = l;
                    while left < right {
                        let middle = (left + right) / 2;
                        if a[middle].0 + a[l].0 == target {
                            right = middle;
                            break;
                        } else if a[middle].0 + a[l].0 < target {
                            left = middle + 1;
                        } else {
                            right = middle;
                        }
                    }
                    k = right;
                } else {
                    let mut left = k;
                    let mut right = l - 1;
                    while left < right {
                        let middle = (left + right + 1) / 2;
                        if a[middle].0 + a[k].0 == target {
                            left = middle;
                            break;
                        } else if a[middle].0 + a[k].0 > target {
                            right = middle - 1;
                        } else {
                            left = middle;
                        }
                    }
                    l = left;
                }
            }
        }
    }
    println!("IMPOSSIBLE");
}

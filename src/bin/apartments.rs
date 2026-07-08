use std::io::Read;
fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let lines = input.trim().lines().collect::<Vec<&str>>();
    let line1 = lines[0]
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    let n = line1[0];
    let m = line1[1];
    let k = line1[2];
    let mut a = lines[1]
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    a.sort();
    let mut b = lines[2]
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    b.sort();
    let mut i: usize = 0;
    let mut j: usize = 0;
    let mut count = 0;
    while i < n as usize && j < m as usize {
        if a[i].checked_sub(k).unwrap_or(0) <= b[j]
        && a[i].checked_add(k).unwrap_or(u32::MAX) >= b[j] {
            count += 1;
            i += 1;
            j += 1;
        } else if a[i].checked_sub(k).unwrap_or(0) > b[j] {
            j += 1
        } else {
            i += 1
        }
    }
    println!("{count}");
}

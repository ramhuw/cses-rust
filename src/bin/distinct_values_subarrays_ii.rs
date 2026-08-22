use std::{collections::HashMap, io::Read};


fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();
    let n: usize = tokens.next().unwrap().parse().unwrap();
    let k: usize = tokens.next().unwrap().parse().unwrap();
    let x: Vec<u32> = tokens.map(|a|a.parse().unwrap()).collect();
    let mut map: HashMap<u32, usize> = HashMap::new();
    let mut left = 0usize;
    let mut right = 0usize;
    let mut ans = 0usize;
    loop {
        while right < n && map.len() <= k {
            ans += right - left;
            *map.entry(x[right]).or_insert(0) += 1;
            right += 1;
        }
        while map.len() > k {
            *map.entry(x[left]).or_insert(0) -= 1;
            if map.get(&x[left]) == Some(&0) {
                map.remove(&x[left]);
            }
            left += 1;
        }
        if right == n {
            ans += right - left;
            break;
        }
    }
    println!("{ans}");
}


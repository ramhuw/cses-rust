use std::{io::Read, collections::HashMap};
fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let n: usize = lines.next().unwrap().parse().unwrap();
    let ks: Vec<u32> = lines.next().unwrap().split_whitespace().map(|c| c.parse::<u32>().unwrap()).collect();
    let mut i: usize = 0;
    let mut j: usize = 0;
    let mut map: HashMap<u32, usize> = HashMap::new();
    let mut ans: usize = 0;
    while j < n {
        if let Some(&index) = map.get(&ks[j]) {
            while i <= index {
                map.remove(&ks[i]);
                i += 1;
            }
        }
        map.insert(ks[j], j);
        j += 1;
        ans = ans.max(j - i);
    }
    println!("{ans}");
}
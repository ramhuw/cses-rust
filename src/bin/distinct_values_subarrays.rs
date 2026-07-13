fn main() {
    use std::{io, collections::HashMap};
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let arr: Vec<u32> = input.trim().split_whitespace().map(|a| a.parse::<u32>().unwrap()).collect();
    let mut map: HashMap<u32, usize> = HashMap::new();
    let mut count: usize = 0;
    let mut left: usize = 0;
    let mut right: usize = 0;
    loop {
        if right == n {
            count += (1 + right - left) * (right - left) / 2;
            break;
        }
        if let Some(&index) = map.get(&arr[right]) {
            count += (2 * right - index - left) * (index - left + 1) / 2;
            while left < index + 1 {
                map.remove(&arr[left]);
                left += 1;
            }
            map.entry(arr[right]).and_modify(|i| *i = right);
        } else {
            map.insert(arr[right], right);
            right += 1;
        }
    }
    println!("{}", count);
}
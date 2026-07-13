use std::io::Read;
use std::collections::HashSet;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let _: u32 = lines.next().unwrap().parse().unwrap();
    let list = lines.next().unwrap().split_whitespace().map(|x| x.parse::<u32>().unwrap());
    let mut count: u32 = 0;
    let mut set: HashSet<u32> = HashSet::new();
    for i in list {
        if i > 0 && set.contains(&(i - 1)) {
            set.remove(&(i - 1));
        } else {
            count += 1;
        }
        set.insert(i);
    }
    println!("{}", count);
}
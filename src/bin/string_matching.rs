use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let sentense: Vec<char> = lines.next().unwrap().chars().collect();
    let pattern: Vec<char> = lines.next().unwrap().chars().collect();
    let mut next = vec![0];
    let mut count = 0;
    for i in 1..pattern.len() {
        let mut j = next[i - 1];
        while j > 0 && pattern[j] != pattern[i] {
            j = next[j - 1];
        }
        if pattern[j] == pattern[i] {
            j += 1;
        }
        next.push(j);
    }
    let mut j: usize = 0;
    for i in 0..sentense.len() {
        while j > 0 && sentense[i] != pattern[j] {
            j = next[j - 1];
        }
        if sentense[i] == pattern[j] {
            j += 1;
            if j >= pattern.len() {
                count += 1;
                j = next[j - 1];
            }
        }
    }

    println!("{:?}", count);
}

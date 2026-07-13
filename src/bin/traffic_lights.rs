fn main() {
    use std::collections::BTreeMap;
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut na = input
        .trim()
        .split_whitespace()
        .map(|a| a.parse::<usize>().unwrap());
    let x = na.next().unwrap();
    input.clear();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut ps = input
        .trim()
        .split_whitespace()
        .map(|a| a.parse::<usize>().unwrap());
    let mut intervals: BTreeMap<usize, usize> = BTreeMap::new();
    intervals.insert(0, x);
    let mut lengths: BTreeMap<usize, usize> = BTreeMap::new();
    lengths.insert(x, 1);
    let mut ans: Vec<String> = vec![];
    while let Some(p) = ps.next() {
        let (&left, &right) = intervals.range(..p).next_back().unwrap();
        let old_length = right - left;
        let a = intervals.entry(left).or_insert(0);
        *a = p;
        intervals.insert(p, right);
        let b = lengths.entry(old_length).or_insert(0);
        *b -= 1;
        if *b == 0 {
            lengths.remove(&old_length);
        }
        let l = lengths.entry(p - left).or_insert(0);
        *l += 1;
        let r = lengths.entry(right - p).or_insert(0);
        *r += 1;
        let d = lengths.last_entry().unwrap();
        ans.push(d.key().to_string());
    }
    println!("{}", ans.join(" "));
}

use std::collections::BTreeMap;
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
    let _ = line1.next().unwrap();
    let _ = line1.next().unwrap();
    let mut hs = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap());
    let mut hbucket: BTreeMap<u32, usize> = BTreeMap::new();
    while let Some(h) = hs.next() {
        let entry = hbucket.entry(h).or_insert(0);
        *entry += 1;
    }
    let mut ts = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap());
    let mut ans: Vec<Option<u32>> = vec![];
    while let Some(t) = ts.next() {
        let mut price: Option<u32> = None;
        if let Some((k, _)) = hbucket.range(..=t).next_back() {
            price = Some(*k);
        }
        if let Some(p) = price {
            let entry = hbucket.entry(p).or_insert(1);
            *entry -= 1;
            if *entry == 0 {
                hbucket.remove(&p);
            }
        }
        ans.push(price);
    }
    fn show(ox: &Option<u32>) -> String {
        match &ox {
            Some(x) => x.to_string(),
            None => String::from("-1"),
        }
    }

    println!(
        "{}",
        ans.iter().map(show).collect::<Vec<String>>().join("\n")
    );
}

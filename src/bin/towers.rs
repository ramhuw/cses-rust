fn main() {
    use std::collections::BTreeMap;
    let mut input: String = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.clear();
    std::io::stdin().read_line(&mut input).unwrap();
    let ki = input.trim().split_whitespace().map(|s| s.parse::<u32>().unwrap());
    let mut towers: BTreeMap<u32, usize> = BTreeMap::new();
    for k in ki {
        let j = towers.range((k+1)..).next();
        if let Some((&key, &value)) = j {
            if value > 0 {
                let a = towers.entry(key).or_insert(0);
                *a -= 1;
            }
            if value == 1 {
                towers.remove(&key);
            }
            
        }
        let b = towers.entry(k).or_insert(0);
        *b += 1;
    }

    println!("{}", towers.iter().fold(0, |acc, a| acc + a.1));
}
use std::collections::BTreeMap;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input = input.trim().to_string();
    let mut stack: Vec<char> = input.chars().collect();
    stack.sort();
    let n = stack.len();
    let mut m: BTreeMap<char, usize> = BTreeMap::new();
    for c in stack {
        *m.entry(c).or_insert(0) += 1;
    }
    let mut max_value = m.clone().into_values().max().unwrap();
    let mut last_key: Option<char> = None;
    let mut ans = String::new();
    for i in 0..n {
        let mut l = n - i;
        let mut max_value = m.clone().into_values().max().unwrap();
        if max_value * 2 - 1 < l {
            let (mut key, mut value) = match m
                .iter_mut()
                .find(|(key, value)| Some(**key) != last_key && **value > 0)
            {
                Some(x) => x,
                None => {
                    println!("-1");
                    return;
                }
            };
            last_key = Some(*key);
            *value -= 1;
            ans += &*key.to_string().as_str();
        } else {
            let (mut key, mut value) = match m
                .iter_mut()
                .find(|(key, value)| **value == max_value && Some(**key) != last_key)
            {
                Some(x) => x,
                None => {
                    println!("-1");
                    return;
                }
            };
            last_key = Some(*key);
            *value -= 1;
            ans += &*key.to_string().as_str();
        }
    }
    print!("{}", ans);
}

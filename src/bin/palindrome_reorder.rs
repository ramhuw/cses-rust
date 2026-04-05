use std::collections::HashMap;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let letter_counts: HashMap<char, usize> = input
        .trim()
        .chars()
        .fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        });
    let mut count_odd = 0;
    let mut odd_key: &char = &' ';
    let mut half_result = String::new();
    for (key, value) in &letter_counts {
        if value % 2 != 0 {
            count_odd += 1;
            odd_key = key;
        } else {
            half_result.push_str(&key.to_string().repeat(value / 2));
        }
        if count_odd > 1 {
            println!("NO SOLUTION");
            return;
        }
    }
    let mut result = if count_odd == 0 {String::new()} else {odd_key.to_string().repeat(letter_counts[odd_key])};
    result = half_result.clone() + &result + &half_result.chars().rev().collect::<String>();
    println!("{}", result);
}

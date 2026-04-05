use std::collections::HashSet;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let input: Vec<char> = input.trim().chars().collect();
    let hash_set = permute(&input);
    let mut result: Vec<String> = hash_set.into_iter().map(|v| v.into_iter().collect()).collect();
    result.sort();
    println!("{}", result.len());
    let result = result.join("\n");
    println!("{}", result);
}

fn permute(input: &Vec<char>) -> HashSet<Vec<char>> {
    let mut hash_set = HashSet::new();
    if input.len() == 1 {
        hash_set.insert(input.clone());
    } else {
        let sub_per = permute(&input[1..].to_vec());
        for sub in sub_per {
            for i in 0..input.len() {
                let mut temp = sub.clone();
                temp.insert(i, input[0]);
                hash_set.insert(temp);
            }
        }
    }
    hash_set
}

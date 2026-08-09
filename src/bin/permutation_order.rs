use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace().map(|x| x.parse::<usize>().unwrap());
    let t = tokens.next().unwrap();
    let mut ans = String::new();
    let mut fact: Vec<usize> = vec![1];
    for i in 1..20 {
        fact.push(i * fact[i-1]);
    }
    for _ in 0..t {
        let label = tokens.next().unwrap();
        if label == 1 {
            let n = tokens.next().unwrap();
            let mut k = tokens.next().unwrap() - 1;
            let mut choices: Vec<usize> = (1..=n).collect();
            for i in 1..=n {
                let j = k / fact[n-i];
                ans.push_str(&(choices[j].to_string() + " "));
                choices.remove(j);
                k -= j * fact[n-i];
            }
            ans.push('\n');
        } else {
            let n = tokens.next().unwrap();
            let mut k: usize = 1;
            let mut choices: Vec<usize> = (1..=n).collect();
            for i in 1..=n {
                let p = tokens.next().unwrap();
                let mut j = 0;
                while choices[j] != p {
                    j += 1;
                }
                k += fact[n-i] * j;
                choices.remove(j);
            }
            ans.push_str(&k.to_string());
            ans.push('\n');
        }
    }
    println!("{ans}");
}
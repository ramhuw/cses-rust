use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap());
    let n = tokens.next().unwrap();
    let m = tokens.next().unwrap();
    let mut cities: Vec<Vec<usize>> = vec![vec![]; n];
    for _ in 0..m {
        let a = tokens.next().unwrap();
        let b = tokens.next().unwrap();
        cities[a - 1].push(b - 1);
        cities[b - 1].push(a - 1);
    }
    let mut parent: Vec<Option<usize>> = vec![None; n];
    let mut visited: Vec<bool> = vec![false; n];
    for i in 0..n {
        if visited[i] {
            continue;
        }
        let mut search: Vec<usize> = vec![i];
        while let Some(j) = search.pop() {
            visited[j] = true;
            for &k in &cities[j] {
                if Some(k) == parent[j] {
                    continue;
                }
                if visited[k] {
                    let mut ans = (k + 1).to_string();
                    let mut l = j;
                    let mut count: usize = 2;
                    while l != k {
                        count += 1;
                        ans.push(' ');
                        ans.push_str(&(l + 1).to_string());
                        l = parent[l].unwrap();
                    }
                    ans.push(' ');
                    ans.push_str(&(k + 1).to_string());
                    println!("{count}\n{ans}");
                    return;
                }

                parent[k] = Some(j);
                search.push(k);
            }
        }
    }
    println!("IMPOSSIBLE");
}

use std::{collections::VecDeque, io::Read};

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines().map(|line| line.split_whitespace().map(|a| a.parse::<usize>().unwrap()));
    let mut nm = lines.next().unwrap();
    let n = nm.next().unwrap();
    let _ = nm.next().unwrap();
    let mut computers: Vec<Vec<usize>> = vec![vec![]; n];
    while let Some(mut line) = lines.next() {
        let a = line.next().unwrap() - 1;
        let b = line.next().unwrap() - 1;
        computers[a].push(b);
        computers[b].push(a);
    }
    let mut steps: Vec<Option<usize>> = vec![None; n];
    let mut searches: VecDeque<(usize, usize)> = VecDeque::from([(0, 1)]);
    let mut found = false;
    while let Some((c, l)) = searches.pop_back() {
        if steps[c].is_none() || steps[c].unwrap() > l {
            steps[c] = Some(l);
            for &d in &computers[c] {
                searches.push_front((d, l + 1));
            }
        }
        if c == n - 1 {
            found = true;
            println!("{}", l);
            break;
        }
    }
    if found {
        let mut ans: VecDeque<String> = VecDeque::new();
        let mut visit = n - 1;
        loop {
            ans.push_front((visit+1).to_string());
            if visit == 0 {
                break;
            }
            for &p in &computers[visit] {
                if steps[p].is_some() && steps[p].unwrap() + 1 == steps[visit].unwrap() {
                    visit = p;
                    break;
                }
            }
        } 
        println!("{}", ans.into_iter().collect::<Vec<String>>().join(" "));
    } else {
        println!("IMPOSSIBLE");
    }

}
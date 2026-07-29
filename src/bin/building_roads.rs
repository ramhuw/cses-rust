use std::{collections::HashSet, io::Read};

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let mut line1 = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|a| a.parse::<usize>().unwrap());
    let n = line1.next().unwrap();
    let _ = line1.next().unwrap();
    let mut cities: Vec<Vec<usize>> = vec![vec![]; n];
    while let Some(line) = lines.next() {
        let mut ab = line
            .split_whitespace()
            .map(|a| a.parse::<usize>().unwrap() - 1);
        let a = ab.next().unwrap();
        let b = ab.next().unwrap();
        cities[a].push(b);
        cities[b].push(a);
    }
    let mut grouped: HashSet<usize> = HashSet::new();
    let mut groups: Vec<HashSet<usize>> = vec![];
    for i in 0..n {
        if grouped.contains(&i) {
            continue;
        }
        let mut group: HashSet<usize> = HashSet::new();
        let mut searches = vec![i];

        while let Some(search) = searches.pop() {
            if !group.contains(&search) {
                group.insert(search);
                grouped.insert(search);
                for &neibor in &cities[search] {
                    searches.push(neibor)
                }
            }
        }
        groups.push(group);
    }
    println!("{}", groups.len() - 1);
    let mut a = groups[0].iter().next().unwrap() + 1;
    for i in 1..groups.len() {
        let b = groups[i].iter().next().unwrap() + 1;
        println!("{} {}", a, b);
        a = b;
    }
}

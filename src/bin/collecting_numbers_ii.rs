use std::collections::HashSet;
use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let n: usize = lines.next().unwrap().split_whitespace().next().unwrap().parse().unwrap();
    let mut list: Vec<usize> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap() - 1)
        .collect();
    let mut count: usize = 0;
    {
        let mut set: HashSet<usize> = HashSet::new();
        for &i in &list {
            if i > 0 && set.contains(&(i - 1)) {
                set.remove(&(i - 1));
            } else {
                count += 1;
            }
            set.insert(i);
        }
    }
    let mut pos: Vec<usize> = vec![0; list.len()];
    for (i, j) in list.iter().enumerate() {
        pos[*j] = i;
    }
    let mut counts: Vec<String> = vec![];
    while let Some(line) = lines.next() {
        let mut ab = line.split_whitespace().map(|x| x.parse::<usize>().unwrap() - 1);
        let mut a = ab.next().unwrap();
        let mut b = ab.next().unwrap();
        if a > b {
            (a, b) = (b, a);
        }
        let x = list[a];
        let y = list[b];
        if x > 0 && pos[x-1] > a && pos[x-1] < b {
            count -= 1;
        }
        if x < n-1 && pos[x+1] > a && pos[x+1] < b {
            count += 1;
        }
        if y > 0 && pos[y-1] >= a && pos[y-1] < b {
            count += 1;
        }
        if y < n-1 && pos[y+1] >= a && pos[y+1] < b {
            count -= 1;
        }
        (list[a], list[b]) = (list[b], list[a]);
        (pos[x], pos[y]) = (pos[y], pos[x]);
        counts.push(count.to_string());
    }
    println!("{}", counts.join("\n"));
}

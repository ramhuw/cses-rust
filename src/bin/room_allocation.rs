use std::{cmp::Reverse, collections::BTreeSet, fmt::Write, io::Read};

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();
    let n = tokens.next().unwrap().parse::<usize>().unwrap();
    let mut intervals: Vec<(u32, u32, usize)> = vec![];
    for i in 0..n {
        intervals.push((
            tokens.next().unwrap().parse::<u32>().unwrap(),
            tokens.next().unwrap().parse::<u32>().unwrap(),
            i,
        ));
    }
    intervals.sort_by_key(|&(a, b, _)| (a, Reverse(b)));
    let mut rooms: BTreeSet<(u32, usize)> = BTreeSet::new();
    let mut arrange: Vec<usize> = vec![0; n];
    let mut l: usize = 0;
    for i in 0..n {
        if rooms.is_empty() || rooms.first().unwrap().0 >= intervals[i].0 {
            l += 1;
            arrange[intervals[i].2] = l;
            rooms.insert((intervals[i].1, l));
        } else {
            let &(k, v) = rooms.first().unwrap();
            rooms.remove(&(k, v));
            rooms.insert((intervals[i].1, v));
            arrange[intervals[i].2] = v;
        }
    }
    let mut ans = rooms.len().to_string();
    ans.push('\n');
    for a in arrange {
        write!(ans, "{} ", a).unwrap();
    }
    println!("{ans}");
}

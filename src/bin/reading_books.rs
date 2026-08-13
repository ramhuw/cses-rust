use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();
    let n: usize = tokens.next().unwrap().parse().unwrap();
    let mut t: Vec<usize> = vec![];
    for _ in 0..n {
        t.push(tokens.next().unwrap().parse().unwrap());
    }
    t.sort();
    let mut readk: Vec<usize> = (0..n).collect();
    let mut readj: Vec<usize> = (0..n).rev().collect();
    let mut timek = 0usize;
    let mut timej = 0usize;
    let mut kv: Option<usize> = None;
    let mut jv: Option<usize> = None;
    while !readk.is_empty() || !readj.is_empty() {
        if (timek <= timej || readj.is_empty()) && !readk.is_empty()  {
            if timek >= timej {
                jv = None;
            }
            let mut k = readk.len() - 1;
            let mut flag = true;
            if jv == Some(readk[k]) {
                if k != 0 {
                    k -= 1;
                } else {
                    timek = timej.max(timek);
                    flag = false;
                    kv = None;
                }
            }
            if flag {
                timek += t[readk[k]];
                kv = Some(readk[k]);
                readk.remove(k);
            }
        } if (timek >= timej || readk.is_empty()) && !readj.is_empty()  {
            if timek <= timej {
                kv = None;
            }
            let mut j = readj.len() - 1;
            let mut flag = true;
            if kv == Some(readj[j]) {
                if j != 0 {
                    j -= 1;
                } else {
                    timej = timek.max(timej);
                    flag = false;
                    jv = None;
                }
            }
            if flag {
                timej += t[readj[j]];
                jv = Some(readj[j]);
                readj.remove(j);
            }
        } 
    }
    println!("{}", timej.max(timek));
}

use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let n: usize = lines.next().unwrap().parse().unwrap();
    let mut movies: Vec<(u32, u32)> = Vec::new();
    while let Some(line) = lines.next() {
        let mut ab = line.split_whitespace().map(|x| x.parse::<u32>().unwrap());
        let a = ab.next().unwrap();
        let b = ab.next().unwrap();
        movies.push((a, b));
    }
    movies.sort_by(|a, b| a.1.cmp(&b.1));
    let mut end = 0;
    let mut i = 0;
    let mut count = 0;
    while i < n {
        if movies[i].0 >= end {
            end = movies[i].1;
            count += 1;
        }
        i += 1;
    }
    println!("{count}");
}
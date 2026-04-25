use std::io;

const P: u32 = 1000000007;

fn main() {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let q: u32 = input.trim().parse().unwrap();
    for _ in 0..q {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let mut iter = line
            .trim()
            .split_whitespace()
            .map(|x| x.to_string().parse::<u32>().unwrap());
        let a = iter.next().unwrap();
        let a = iter.next().unwrap();
        todo!()
    }
}

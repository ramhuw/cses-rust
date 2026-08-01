use std::io::{self, Write, Read};

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input
        .split_whitespace();
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    let t: usize = tokens.next().unwrap().parse().unwrap();
    for _ in 0..t {
        let x1 = tokens.next().unwrap().parse::<i64>().unwrap();
        let y1 = tokens.next().unwrap().parse::<i64>().unwrap();
        let x2 = tokens.next().unwrap().parse::<i64>().unwrap();
        let y2 = tokens.next().unwrap().parse::<i64>().unwrap();
        let x3 = tokens.next().unwrap().parse::<i64>().unwrap();
        let y3 = tokens.next().unwrap().parse::<i64>().unwrap();
        let v = (x2-x1)*(y3-y1) - (y2-y1)*(x3-x1);
        if v == 0 {
            writeln!(out, "TOUCH").unwrap();
        } else if v > 0 {
            writeln!(out, "LEFT").unwrap();
        } else {
            writeln!(out, "RIGHT").unwrap();
        }
    }
}
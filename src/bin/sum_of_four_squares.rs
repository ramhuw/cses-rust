use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap());
    let t = tokens.next().unwrap();
    let mut squares: Vec<Option<(usize, usize)>> = vec![None; 10000001];
    for i in 0.. {
        let a = i * i;
        if 2 * a > 10000000 {
            break;
        }
        for j in i.. {
            let b = j * j;
            if a + b > 10000000 {
                break;
            }
            if squares[a + b].is_none() {
                squares[a + b] = Some((i, j));
            }
        }
    }
    let mut ans = String::new();
    for _ in 0..t {
        let n = tokens.next().unwrap();
        let mut k = 0;
        let mut l = n;
        while squares[k].is_none() || squares[l].is_none() {
            k += 1;
            l -= 1;
        }
        let (a, b) = squares[k].unwrap();
        let (c, d) = squares[l].unwrap();
        ans.push_str(&format!("{} {} {} {}\n", a, b, c, d));
    }
    println!("{ans}");
}

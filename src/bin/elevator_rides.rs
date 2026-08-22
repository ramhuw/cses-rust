use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();
    let n: u8 = tokens.next().unwrap().parse().unwrap();
    let x: u32 = tokens.next().unwrap().parse().unwrap();
    let w: Vec<u32> = tokens.map(|v| v.parse::<u32>().unwrap()).collect();
    let mut d: Vec<(u8, u32)> = vec![(0, x)];
    for i in 1usize..(1 << n) {
        let mut r = 20;
        let mut j = 0;
        while 1 << j <= i  {
            if 1 << j & i != 0 {
                if d[1 << j ^ i].1 + w[j] <= x {
                    r = r.min(d[1 << j ^ i].0)
                } else {
                    r = r.min(d[1 << j ^ i].0 + 1);
                }
            }
            j += 1;
        }
        let mut m = x;
        j = 0;
        while 1 << j <= i {
            if 1 << j & i != 0 {
                if d[1 << j ^ i].1 + w[j] <= x && d[1 << j ^ i].0 == r {
                    m = m.min(d[1 << j ^ i].1 + w[j]);
                } else if d[1 << j ^ i].1 + w[j] > x && d[1 << j ^ i].0 + 1 == r {
                    m = m.min(w[j]);
                }
            }
            j += 1;
        }
        d.push((r, m));
    }
    println!("{:?}", d.last().unwrap().0);
}

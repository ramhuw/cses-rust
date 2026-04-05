fn main() {
    use std::io;
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut n: u64 = input.trim().parse().unwrap();
    let mut result: u64 = 0;
    let mut bit_count: u64 = 0;
    let mut m: u32 = u64::BITS - n.leading_zeros();
    let mut v: Vec<u64> = Vec::new();
    v.push(0);
    for i in 1..m {
        if i == 1 {
            v.push(1);
        } else {
            v.push(v[i as usize - 1] * 2 + (1 << (i - 1)));
        }
    }
    while m != 0 {
        if (n >> (m - 1)) & 1 == 1 {
            result += v[m as usize - 1];
            result += bit_count * (1 << (m - 1));
            bit_count += 1;
        }
        n = n | (1 << m);
        m -= 1;
    }
    result += bit_count;
    println!("{}", result);
}

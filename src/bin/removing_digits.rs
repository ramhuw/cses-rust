fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();
    let mut v: Vec<usize> = vec![0];
    for i in 1..=n {
        let mut u = vec![];
        let mut j = i;
        while j != 0 {
            u.push(j%10);
            j /= 10;
        }
        let mut a = usize::MAX;
        for j in u {
            if i >= j && j != 0 {
                a = a.min(v[i-j] + 1);
            }
        }
        v.push(a);
    }
    println!("{}", v.last().unwrap());
}
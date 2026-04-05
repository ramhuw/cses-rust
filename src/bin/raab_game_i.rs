fn main() {
    use std::io;
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n: i32 = n.trim().parse().unwrap();
    for _ in 0..n {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input: Vec<i32> = input.trim().split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let n = input[0];
        let a = input[1];
        let b = input[2];
        if  a + b > n || (a == 0 && b != 0) || (a != 0 && b == 0) {
            println!("NO");
        } else {
            let mut av: Vec<i32> = Vec::new();
            let mut bv: Vec<i32> = Vec::new();
            println!("YES");
            for i in b+1..=a+b {
                av.push(i);
                bv.push(i - b);
            }
            for i in 1..=b {
                av.push(i);
                bv.push(i + a);
            }
            for i in a+b+1..n+1 {
                av.push(i);
                bv.push(i);
            }      
            println!("{}", av.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
            println!("{}", bv.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
        }
    }
}

fn main() {
    use std::io;
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read line");
    let n: u32 = n.trim().parse().expect("Please type a number!");
    let a = 2u32.pow(n);
    let _ = (0..a).into_iter().map(|i| {
        i ^ (i >> 1)
    })
        .for_each(|i| {
        println!("{:0width$b}", i, width = n as usize);
    });
}

fn main() {
    use std::io;
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read line");
    let n: i64 = n.trim().parse().expect("Please type a number!");
    two_nights(n);
}

fn two_nights(n: i64) -> i64 {
    let mut result: i64 = 0;
    if n == 1 {
        result = 0;
        println!("{}", result);
    } else if n == 2{
        result = two_nights(1) + 6;
        println!("{}", result);
    } else {
        result = two_nights(n - 1) + (2*n - 1)*(n-1)*(n-1) + (2*n-1)*(n-1)
        - 2*((n - 2) + (n - 1) + (n - 2) + (n - 1)) + 4 ;
        println!("{}", result);
    }
    return result;
}

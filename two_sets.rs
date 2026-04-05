fn main() {
    use std::io;
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Fail to read");
    let n:i32 = n.trim().parse().unwrap();
    if n % 4 == 0 {
        println!("YES");
        println!("{}", n / 2);
        for i in (1..=(n/2)).step_by(2) {
            print!("{} {} ", i, n - i + 1);
        }
        print!("\n");
        println!("{}", n / 2);
    
        for i in (2..=n/2).step_by(2) {
            print!("{} {} ", i, n - i + 1);
        }
        print!("\n");
    } else if n % 4 == 3 {
        println!("YES");
        println!("{}", (n-3) / 2 + 2);
        for i in (4..=((n-3)/2) + 3).step_by(2) {
            print!("{} {} ", i, n + 3 - i + 1);
        }
        print!("{} {}", 1, 2);
        print!("\n");
        println!("{}", (n-3) / 2 + 1);
    
        for i in (5..=((n-3)/2) + 3).step_by(2) {
            print!("{} {} ", i, n + 3 - i + 1);
        }
        print!("{} ", 3);
        print!("\n");
    } else {
        println!("NO");
    }
}

fn main() {
    let mut n = String::new();
    std::io::stdin()
    .read_line(&mut n)
    .expect("Failed to read line");
    let n: i32 = n.trim().parse().expect("Please type a number!");
    if n % 2 == 0 && n >= 6 {
        for i in (1..=n-1).step_by(2) {
            print!("{} ", i);
        }
        for i in (2..=n-2).step_by(2) {
            print!("{} ", i);
        }
        print!("{}", n);
    } else if n % 2 == 1 && n >= 5 {
        for i in (1..=n).step_by(2) {
            print!("{} ", i);
        }
        for i in (2..=n-3).step_by(2) {
            print!("{} ", i);
        }
        print!("{}", n-1);
    } else if n == 1 {
        print!("1");
    } else if n == 4 {
        print!("2 4 1 3");
    } else {
        println!("NO SOLUTION");
    }
}

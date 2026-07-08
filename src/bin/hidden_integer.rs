use std::io;

fn main() {
    let mut left: u32 = 1;
    let mut right: u32 = 1000000000;
    loop {
        if left >= right {
            println!("! {}", right);
            break;
        }
        let middle = (left + right) / 2;
        println!("? {}", middle);
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if input.trim() == "YES" {
            left = middle + 1;
        } else {
            right = middle
        }
    }
}
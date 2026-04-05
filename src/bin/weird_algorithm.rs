use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut input: u64 = input.trim().parse().expect("Please type a number!");
    print!("{}", &input);
    loop {
        if input == 1 {
            break;
        } if &input % 2 == 0 {
            input /= 2;
            print!(" {}", &input);
        } else if &input % 2 == 1 {
            input = input * 3 + 1;
            print!(" {}", &input);
        } 
    }
}

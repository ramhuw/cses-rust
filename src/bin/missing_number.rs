fn main() {
    use std::io;
    let mut n = String::new();
    io::stdin().read_line(&mut n)
    .expect("Failed to read line");
    let n = n.trim().parse::<i32>().expect("Not a number");
    let mut numbers = String::new();
    io::stdin().read_line(&mut numbers)
    .expect("Failed to read line");
    let numbers: Vec<i32> = numbers
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i32>().expect("Not a number"))
        .collect();
    let mut found = vec!(false; n as usize);
    for number in numbers {
        found[(number - 1) as usize] = true;
    }
    let missing_numbers = found.iter().position(|&x| !x).unwrap() + 1;
    println!("{}", missing_numbers);

}

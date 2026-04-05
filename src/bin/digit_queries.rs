use std::io::{Read, stdin};

fn main() {
    let mut input: String = String::new();
    stdin().read_to_string(&mut input).unwrap();
    input = input.trim().to_string();
    let mut lines = input.lines();
    let _ = lines.next();
    while let Some(line) = lines.next() {
        let mut query: u128 = line.trim().parse().unwrap();
        let mut digit: u32 = 0;
        while let Some(reduced) = query.checked_sub(9 * 10u128.pow(digit) * (digit as u128 + 1)) {
            if reduced == 0 {
                break;
            }
            query = reduced;
            digit += 1;
        }
        let mut number: u128 = 10u128.pow(digit) + query / (digit as u128 + 1);
        let mut index: u128 = query % (digit as u128 + 1);
        if index == 0 {
            number = number.checked_sub(1).unwrap();
            index = digit as u128;
        } else {
            index = index.checked_sub(1).unwrap();
        }
        println!(
            "{}",
            number.to_string().chars().collect::<Vec<char>>()[index as usize]
        );
    }
}
